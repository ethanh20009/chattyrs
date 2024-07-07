use crate::{commands::run_ask, llm::engine::LlmEngine};
use crate::{
    commands::{
        error::{Error, Result},
        weigh_in::run_weigh_in,
    },
    environment::Environment,
};
use serenity::{
    all::{
        CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseFollowup,
        CreateInteractionResponseMessage, EventHandler, Http, Interaction, Ready,
    },
    async_trait,
    prelude::*,
};

pub struct Handler {
    llm_engine: LlmEngine,
    http_client: serenity::http::Http,
    environment: Environment,
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    // async fn message(&self, ctx: Context, msg: Message) {}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content: Result<String> = match command.data.name.as_str() {
                "ask" => {
                    self.send_defer_message(&command, &ctx).await;
                    run_ask(&command.data.options(), &self.llm_engine).await
                }
                "weigh-in" => {
                    self.send_defer_message(&command, &ctx).await;
                    run_weigh_in(
                        &command,
                        &self.llm_engine,
                        &self.http_client,
                        &self.environment,
                    )
                    .await
                }
                _ => Err(Error::CommandNotImplemented),
            };

            let response_message = match &content {
                Ok(reply) => reply,
                Err(err) => {
                    println!("Interaction execution failed, reason: {}", err);
                    "Command failed to execute, please try again later"
                }
            };

            if let Err(why) = self
                .send_message_in_chunks(response_message, &command, &ctx)
                .await
            {
                println!("Sending command response failed {why:?}");
                if let SerenityError::Model(ModelError::MessageTooLong(size)) = why {
                    println!("size: {}, {}", size, content.unwrap_or("".to_string()))
                }
                let _ = command.delete_response(&ctx.http).await;
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

impl Handler {
    pub fn new(
        environment: &Environment,
        http_client: Http,
    ) -> std::result::Result<Handler, crate::error::Error> {
        Ok(Handler {
            llm_engine: LlmEngine::new(environment)?,
            http_client,
            environment: environment.clone(),
        })
    }

    async fn send_message_in_chunks(
        &self,
        message: &str,
        command: &CommandInteraction,
        ctx: &Context,
    ) -> std::result::Result<(), serenity::Error> {
        let messages: Vec<&str> = message.split("\n\n").collect();

        for message in messages {
            if message.chars().count() > 0 {
                let data = CreateInteractionResponseFollowup::new().content(message);
                command.create_followup(&ctx.http, data).await?;
            }
        }

        Ok(())
    }

    async fn send_defer_message(&self, command: &CommandInteraction, ctx: &Context) {
        if let Err(why) = command
            .create_response(
                &ctx,
                CreateInteractionResponse::Defer(
                    CreateInteractionResponseMessage::new()
                        .content("Working on my response. Please wait"),
                ),
            )
            .await
        {
            println!("Failed to defer ask {why:?}");
        }
    }
}
