use reqwest::Client;

use crate::methods;

pub async fn run(method: &String, url: &String) {
  let client = Client::new();
  let base_url = "https://jsonplaceholder.typicode.com";
  let url = format!("{}{}", base_url, url);

  actions(method, client, url).await;
}

async fn actions(method: &String, client: Client, url: String) {
  if method == "get" {
    methods::get(client, url).await;
  }
}
