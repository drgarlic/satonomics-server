use reqwest::Client;
use serde_json::Value;

// pub async fn fetch_html(client: &Client, url: &str) -> color_eyre::Result<String> {
//     Ok(client.get(url).send().await?.text().await?)
// }

pub async fn fetch_json(
    client: &Client,
    url: &str,
    body: &Option<String>,
) -> color_eyre::Result<Value, reqwest::Error> {
    let builder;

    if let Some(body) = body {
        builder = client.post(url).body(body.clone());
    } else {
        builder = client.get(url);
    }

    let response = builder.header("User-Agent", "curl").send().await?;

    response.json::<Value>().await
}
