use reqwest::{Client, StatusCode};

use colored::Colorize;

use tokio::{
	 fs::File, io::{AsyncBufReadExt, BufReader}, sync::Semaphore, task, time::{sleep, Duration}
	 //	TODO: INTEGRATE runtime::Runtime For Async Processes Later
};

// TODO: INTEGRATE use std::collections::HashMap; For Parsing JSON Later

use std::sync::Arc;

use clap::{Arg, App};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	 let matches = App::new("PathFinder")
		  .version("1.0")
		  .author("Burak Gazi Chetin")
		  .about("A fork of Dirbuster")
		  .arg(Arg::new("url")
				 .long("url")
				 .takes_value(true)
				 .required(true)
				 .help("The base URL"))
		  .arg(Arg::new("wordlist")
				 .long("wordlist")
				 .takes_value(true)
				 .required(true)
				 .help("Path to wordlist"))
		  .get_matches();
	 
	 let url = matches.value_of("url").unwrap();
	 let wordlist = matches.value_of("wordlist").unwrap();

	 let client = Client::new();
	 
	 let file = File::open(wordlist).await?;
	 let reader = BufReader::new(file);
	 let mut lines = reader.lines();

	 let semaphore = Arc::new(Semaphore::new(1000));

	 let mut handles = Vec::new();
	 
	 while let Ok(Some(path)) = lines.next_line().await {
		  let temp_url = format!("{}/{}", url, path.trim());
		  let client = client.clone();

		  let semaphore = Arc::clone(&semaphore);

		  let handle = task::spawn(async move {

				let permit = semaphore.acquire().await.unwrap();
				
				match client.get(&temp_url).send().await {
					 Ok(response) => {
						  match response.status() {							
								StatusCode::OK => {
									 // Handle successful response (200)
									 println!("{}",
												 format!("Status Code: 200 OK: {temp_url}").green());
								}
								StatusCode::NOT_FOUND => {
									 // Handle "Not Found" (404)
									 println!("{}",
												 format!("Status Code: 404 NOT FOUND : {temp_url}").red());
								}
								StatusCode::FORBIDDEN => {
									 // Handle "Forbidden" (403)
									 println!("{}",
										  format!("Status Code: 403 FORBIDDEN : {temp_url}").yellow());
								}
								StatusCode::INTERNAL_SERVER_ERROR => {
									 // Handle "Internal Server Error" (500)
									 println!("{}",
												 format!("Status Code: 500 INTERNAL SERVER ERROR : {temp_url}").black());
								}
								// Catch all for other status codes
								_ => {
									 println!("Status Code: {} : {temp_url}", response.status());
								}
								
						  };
					 }
					 Err(err) => {
						  eprintln!("ERROR: Failed to fetch {temp_url}: {err}");
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
