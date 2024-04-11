use mongodb::{
    Client,
    error::Result,
    options::ClientOptions
};

pub async fn setup() -> Result<Client> {
    let mongo_url: &str = "mongodb://localhost:27017";

    let mut client_options: ClientOptions = ClientOptions::parse(mongo_url).await?;
    client_options.app_name = Some("LemCom".to_string());

    let client = Client::with_options(client_options)?;
    Ok(client)
}