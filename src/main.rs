use chattyrs::commands::error::Result;
use chattyrs::commands::{self, get_commands, run_ask};
use chattyrs::environment::{get_environment, Environment};
use serenity::all::{ApplicationId, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Interaction;
use serenity::prelude::*;
use serenity::{async_trait, http};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content: Result<String> = match command.data.name.as_str() {
                "ask" => run_ask(&command.data.options()),
                _ => Err(commands::error::Error::CommandNotImplemented),
            };

            if let Ok(reply) = content {
                let data = CreateInteractionResponseMessage::new().content(reply);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Sending command response failed {why:?}");
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

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
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&environment.discord_token, intents)
        .event_handler(Handler)
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
