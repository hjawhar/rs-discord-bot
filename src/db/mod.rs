use mongodb::{
    options::{self, ClientOptions},
    results::DatabaseSpecification,
    Client
};

pub async fn connect_db() -> mongodb::error::Result<()> {
    println!("Connecting to DB");
    let client_options: options::ClientOptions;
    match ClientOptions::parse("mongodb://localhost:27017").await {
        Ok(res) => client_options = res,
        Err(err) => panic!("Couldn't parse URL: {}", err),
    }

    let client: Client;
    match Client::with_options(client_options) {
        Ok(res) => client = res,
        Err(err) => panic!("Couldn't connected to client: {}", err),
    }

    println!("Successfully connected to DB");
    let dbs: Vec<DatabaseSpecification>;
    match client.list_databases(None, None).await {
        Ok(res) => dbs = res,
        Err(err) => panic!("Couldn't fetch DBs: {}", err),
    }
    
    println!("Listing all available databases...");
    // List the names of the databases in that deployment.
    for db_name in dbs {
        println!("DB name: {}", db_name.name);
    }
    Ok(())
}
