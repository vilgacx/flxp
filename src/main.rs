use confy;
use reqwest;
use reqwest::multipart::{Form, Part};
use serde_derive::{Deserialize, Serialize};
use std::env::args;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
struct FlxpConfig {
    pastebin_api_key: String,
}

impl Default for FlxpConfig {
    fn default() -> Self {
        FlxpConfig {
            pastebin_api_key: "".to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    for (index, arg) in args.iter().enumerate() {
        if arg == "-p" {
            pastebin(&args[index + 1]);
        } else if arg == "-f" {
            zeroxzero(&args[index + 1]);
        } else if arg == "--pbk" {
            set_key(&args[index + 1]).expect("Error");
        }
    }
}

fn set_key(key: &String) -> Result<(), confy::ConfyError> {
    let data = FlxpConfig {
        pastebin_api_key: key.to_string(),
    };
    confy::store("flxp", None, &data)?;
    Ok(())
}

fn get_key() -> FlxpConfig {
    let config: FlxpConfig = match confy::load("flxp", None) {
        Ok(key) => key,
        Err(_e) => panic!("Error while fetching key"),
    };
    config
}

#[tokio::main]
async fn pastebin(text: &String) {
    let key: String = get_key().pastebin_api_key;
    let params = [
        ("api_dev_key", key.to_string()),
        ("api_paste_code", text.to_string()),
        ("api_option", "paste".to_string()),
    ];
    let client = reqwest::Client::new();
    let res = client.post("https://pastebin.com/api/api_post.php")
        .form(&params)
        .send()
        .await
        .unwrap();


    match res.status() {
        reqwest::StatusCode::OK => { 
            println!("Response OK");
            let result = res.text().await;
            let result = result.unwrap();
            println!("{}", &result.as_str());
            println!("https://pastebin.com/raw/{}", &result.as_str()[21..]);
        }
        _ => println!("Error"),
    };
}

#[tokio::main]
async fn zeroxzero(path: &String) {
    match Path::new(&path).is_file() {
        true => {
            let file_name = String::from(Path::new(&path).file_name().unwrap().to_str().unwrap());

            let file = fs::read(&path).await.unwrap();
            let file_part = Part::bytes(file).file_name(file_name);
            let form = Form::new().part("file", file_part);
            
            let client = reqwest::Client::new();

            let res = client.post("https://0x0.st")
                .multipart(form)
                .send()
                .await
                .unwrap();

            match res.status() {
                reqwest::StatusCode::OK => {
                    println!("Response OK");
                    let result = res.text();
                    let final_result = result.await.unwrap();
                    println!("{}", &final_result.as_str());
                }
                _ => println!("Error"),
            };
        }
        false => println!("File doesn't exist"),
    }
}
