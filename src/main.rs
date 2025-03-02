use reqwest::{Client, StatusCode};
use colored::Colorize;
use tokio::{
	 fs::{File, OpenOptions},
	 io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
	 sync::{Mutex, Semaphore},
	 task,
};

use std::{sync::Arc};
use clap::{Arg, App};

struct RunArgs {
    pub url: String,
    pub wordlist: String,
    pub output: String,
	 pub status_codes: Vec<String>,
	 pub debug: bool
}

fn get_args() -> RunArgs {
	 let matches = App::new("PathFinder")
		  .version("1.0")
		  .author("Burak Gazi Chetin")
		  .about("A fork of Dirbuster")
		  .arg(Arg::new("url")
				 .short('u')
				 .long("url")
				 .takes_value(true)
				 .required(true)
				 .help("The base URL"))
		  .arg(Arg::new("wordlist")
				 .short('w')
				 .long("wordlist")
				 .takes_value(true)
				 .required(true)
				 .help("Path to wordlist"))
		  .arg(Arg::new("output")
				 .short('o')
				 .long("output")
				 .takes_value(true)
				 .required(false)
				 .help("Saves the log into the file that's given"))
		  .arg(Arg::new("status_codes")
				 .short('c')
				 .long("status_codes")
				 .takes_value(true)
				 .required(false)
				 .default_missing_value("NO_CODE")
				 .default_value("NO_CODE")
				 .help("only prints the specified status codes codes should be listed like 200,400,500"))
		  .arg(Arg::new("debug-mode")
				 .long("debug")
				 .takes_value(false)
				 .default_missing_value("0")
				 .default_value("1")
				 .required(false)
				 
		  )
		  .get_matches();
	 
	 let url = matches
		  .value_of("url")
		  .unwrap()
		  .to_owned();
	 
	 let wordlist = matches.value_of("wordlist")
		  .unwrap()
		  .to_owned();
	 
	 let status_codes: Vec<String> = matches.value_of("status_codes")
		  .unwrap()
		  .to_owned().split(",").map(|v| v.to_string()).collect();

	 let output = matches.value_of("output")
		  .unwrap_or("")
		  .to_owned();

	 let debug_raw = matches.value_of("debug-mode")
		  .unwrap()
		  .to_owned();
	 let debug: bool;

	 if debug_raw == "1" {
		  debug = true;
	 } else {
		  debug = false; 
	 }
	 

	 RunArgs { url, wordlist, status_codes, output, debug}
}





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

	 let args = get_args();

	 let client = Client::new();

	 let file = File::open(args.wordlist).await?;
	 let reader = BufReader::new(file);
	 let mut lines = reader.lines();

	 let semaphore = Arc::new(Semaphore::new(80));

	 let mut handles = Vec::new();

	 let log_writer = match args.output.as_str() {
		  "" => 	None,
		  path => {
				let file = OpenOptions::new()
					 .create(true)
					 .append(true)
					 .open(path)
					 .await?;
				Some(Arc::new(Mutex::new(file)))
		  }
	 };

	 
	 while let Ok(Some(path)) = lines.next_line().await {
		  let temp_url = format!("{}/{}", args.url, path.trim());
		  let client = client.clone();

		  let semaphore = Arc::clone(&semaphore);
		  let status_codes = args.status_codes.clone();
		  let log_writer = log_writer.clone();

		  let handle = task::spawn(async move {

				let _permit = semaphore.acquire().await.unwrap();
				
				match client.get(&temp_url).send().await {
					 Ok(response) => {
						  if status_codes[0] == "NO_CODE" || status_codes.contains(&response.status().as_str().to_string()) {
								let msg = format!("Status Code: {}: {temp_url}\n", response.status());
								if let Some(writer) = log_writer {
									 let mut writer = writer.lock().await;
									 let _ =  writer.write_all(msg.as_bytes()).await;
								}
								
								match response.status()  {							
									 StatusCode::OK => {
										  // Handle successful response (200)
										  print!("{}", format!("{}", msg).green());
									 }
									 StatusCode::NOT_FOUND => {
										  // Handle "Not Found" (404)
										  print!("{}", format!("{}", msg).red());
									 }
									 StatusCode::FORBIDDEN => {
										  // Handle "Forbidden" (403)
										  print!("{}", format!("{}", msg).yellow());
									 }
									 StatusCode::INTERNAL_SERVER_ERROR => {
										  // Handle "Internal Server Error" (500)
										  print!("{}", format!("{}", msg).black());
									 }
									 // Catch all for other status codes
									 _ => {
										  print!("{msg}");
									 }
									 
								};
						  }
					 }
					 
					 Err(err) => {
						  if args.debug {
								eprintln!("ERROR: Failed to fetch {temp_url}: {err}");
						  }

					 }
				}
		  });

		  handles.push(handle);
		  
	 }

	 for handle in handles {
		  handle.await?;
	 }
	 Ok(())
}
