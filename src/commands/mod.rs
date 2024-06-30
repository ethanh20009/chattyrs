mod ask;
pub mod error;

use serenity::all::CreateCommand;

use crate::{commands::ask::register_ask, environment::Environment};

pub fn get_commands(environment: &Environment) -> Vec<CreateCommand> {
    vec![register_ask(environment)]
}

pub use ask::run_ask;
