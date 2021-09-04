use dotenv::dotenv;
use serenity::model::prelude::PresenceUpdateEvent;
use std::{env, path::Path};

use serenity::{
    async_trait,
    http::AttachmentType,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

async fn handle_ping(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}

async fn build_message(ctx: Context, msg: Message) {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Hello, World!");
            m.embed(|e| {
                e.title("This is a title");
                e.description("This is a description");
                e.image("attachment://ferris_eyes.png");
                e.fields(vec![
                    ("This is the first field", "This is a field body", true),
                    (
                        "This is the second field",
                        "Both of these fields are inline",
                        true,
                    ),
                ]);
                e.field(
                    "This is the third field",
                    "This is not an inline field",
                    false,
                );
                e.footer(|f| {
                    f.text("This is a footer");

                    f
                });

                // Add a timestamp for the current time
                // This also accepts a rfc3339 Timestamp
                // e.timestamp(chrono::Utc::now());

                e
            });
            m.add_file(AttachmentType::Path(Path::new("./ferris_eyes.png")));
            m
        })
        .await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            handle_ping(ctx, msg).await;
        } else if msg.content == "!halo" {
            build_message(ctx, msg).await;
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn presence_update(&self, _ctx: Context, _new_data: PresenceUpdateEvent) {
        // When a user's status is updated, an event is triggered
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
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
