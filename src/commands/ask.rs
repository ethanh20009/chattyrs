use serenity::all::CreateCommand;

use crate::environment::Environment;

pub fn register_ask(environment: &Environment) -> CreateCommand {
    CreateCommand::new("ask").description(format!("Ask {} a question", environment.bot_name))
}
