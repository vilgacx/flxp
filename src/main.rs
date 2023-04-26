use confy;
use reqwest;
use serde_derive::{Deserialize, Serialize};
use std::env::args;

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
        Err(e) => panic!("Error {}", e),
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
    let res = client
        .post("https://pastebin.com/api/api_post.php")
        .form(&params)
        .send()
        .await
        .unwrap();

    println!("{}", res.status());

    match res.status() {
        reqwest::StatusCode::OK => {
            let result = res.text().await;
            println!("Response");
            println!("{:?}", result);
        }
        _ => println!("Error"),
    };
}
