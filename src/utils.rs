
pub async fn get_random_coffee_img() -> Result<String, reqwest::Error> {

  // reqwest is a HTTP Client
  let res = reqwest::get("https://coffee.alexflipnote.dev/random.json").await?;

  let body = res.text().await?;

  Ok(body)
  
}


