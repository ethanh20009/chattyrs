use chattyrs::commands::get_commands;
use chattyrs::environment::{get_environment, Environment};
use chattyrs::handler::Handler;
use serenity::all::ApplicationId;
use serenity::http;
use serenity::prelude::*;

async fn setup_slash_commands(environment: &Environment) {
    let http_serenity = http::Http::new(&environment.discord_token);
    http_serenity.set_application_id(ApplicationId::new(1256701007249936568));
    serenity::model::application::Command::set_global_commands(
        http_serenity,
        get_commands(environment),
    )
    .await
    .expect("Failed to set global commands");
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let environment = get_environment().unwrap();
    println!("Loaded environment {environment:?}");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&environment.discord_token, intents)
        .event_handler(Handler::new(&environment).expect("Failed to create handler"))
        .await
        .expect("Err creating client");

    setup_slash_commands(&environment).await;

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
