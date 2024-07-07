mod ask;
pub mod error;
pub mod weigh_in;

use serenity::all::CreateCommand;

use crate::{
    commands::{ask::register_ask, weigh_in::register_weigh_in},
    environment::Environment,
};

pub fn get_commands(environment: &Environment) -> Vec<CreateCommand> {
    vec![register_ask(environment), register_weigh_in(environment)]
}

pub use ask::run_ask;
