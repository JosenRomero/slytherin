
pub async fn get_random_coffee_img() -> Result<String, reqwest::Error> {

  // reqwest is a HTTP Client
  let res = reqwest::get("https://coffee.alexflipnote.dev/random.json").await?;

  let body = res.text().await?;

  // // serde_json is a JSON serialization file format
  let img: serde_json::Value = serde_json::from_str(&body).expect("Error serializing to JSON");
  
  Ok(img["file"].to_string())
  
}


