use std::env::{ args, var };
use dotenv::dotenv;
use reqwest;

fn main() {
    dotenv().ok();
    let args: Vec<String> = args().collect();
    for (index , arg) in args.iter().enumerate() {
        if arg == "-p" {
            pastebin(&args[index + 1]);
        }
    }
}

#[tokio::main]
async fn pastebin(text: &String) { 
    match var("PASTBIN_API_KEY") {
        Ok(key) => {
            let params = [
                ("api_dev_key",key),
                ("api_paste_code",text.to_string()),
                ("api_option","paste".to_string())
            ];
            let client = reqwest::Client::new();
            let res = client.post("https://pastebin.com/api/api_post.php")
                .form(&params).send().await.unwrap();

            println!("{}",res.status());

            match res.status() {
                reqwest::StatusCode::OK => {
                    let result = res.text().await;
                    println!("Response");
                    println!("{:?}",result);
                },
                _ => println!("Error")
            };
        },
        Err(_e) => println!("Error")
    } 
}
