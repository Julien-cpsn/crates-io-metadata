use reqwest::{Client, Error};
use crate::types::CratesIoResponse;

pub async fn scrap(crate_name: impl AsRef<str>) -> Result<CratesIoResponse, Error> {
    let response = Client::new()
        .get(&format!("https://crates.io/api/v1/crates/{}", crate_name.as_ref()))
        .header("user-agent", "crates-io-metadata")
        .send()
        .await?;

    // Debug purpose
    /*
    println!("{}", &response.text().await.unwrap());

    return Ok(CratesIoResponse::default());
    */

    response.json::<CratesIoResponse>().await
}
