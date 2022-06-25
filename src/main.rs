mod db;
mod discord;

use std::env;

use db::connect_db;
use discord::Handler;
use dotenv::dotenv;

use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok(); 
    println!("------------------------");
    let _db = connect_db().await;
    println!("------------------------");
    println!("Bot is connecting...");
    init_discord().await;
}

async fn init_discord() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
