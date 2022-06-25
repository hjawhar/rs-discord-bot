use mongodb::{options::ClientOptions, Client};

pub async fn connect_db() -> mongodb::error::Result<()> {
    println!("Connecting to DB");
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    println!("Successfully connected to DB");
    let dbs = client.list_databases(None, None).await?;
    println!("Listing all available databases...");
    // List the names of the databases in that deployment.
    for db_name in dbs {
        println!("DB name: {}", db_name.name);
    }
    Ok(())
}
