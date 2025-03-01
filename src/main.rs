use tokio::{
	 fs,
//	TODO: INTEGRATE runtime::Runtime For Async Processes Later
};

// TODO: INTEGRATE use std::collections::HashMap; For Parsing JSON Later

use clap::{Arg, App};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	 let matches = App::new("PathFinder")
		  .version("1.0")
		  .author("Burak Gazi Chetin")
		  .about("A fork of Dirbuster")
		  .arg(Arg::with_name("url")
				 .long("url")
				 .takes_value(true)
				 .required(true)
				 .help("The base URL"))
		  .arg(Arg::with_name("wordlist")
				 .long("wordlist")
				 .takes_value(true)
				 .required(true)
				 .help("Path to wordlist"))
		  .get_matches();
	 
	 let url = matches.value_of("url").unwrap();
	 let wordlist = matches.value_of("wordlist").unwrap();
	 
	 let binding = fs::read_to_string(wordlist)
		  .await.expect("ERROR: Bad URL");

	 let wordlist_content = binding.lines();

	 for path in wordlist_content {
		  let temp_url = format!("{}/{}", url, path.trim());
		  let resp = reqwest::get(temp_url.clone()).await;

		  match resp {
				Ok(response) => {
					 println!("{temp_url}: Status Code: {}", response.status());
				}
				Err(err) => {
					 eprintln!("ERROR: Failed to fetch {temp_url}: {err}");
				}
		  }
	 }
	 
	 Ok(())

}
