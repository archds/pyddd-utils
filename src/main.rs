use std::fmt::Display;

use inquire::Select;

mod workflow;
mod rules;

type ApplicationError = &'static str;

pub enum Action {
    NewWorkflow,
    ExistenceRules
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewWorkflow => write!(f, "New workflow"),
            Self::ExistenceRules => write!(f, "Generate existence rules")
        }
    }
}

fn main() {
    let options: Vec<Action> = vec![Action::NewWorkflow];

    let result = Select::new("Choose command", options)
        .prompt()
        .map_err(|_| "Invalid selection")
        .and_then(|cmd| match cmd {
            Action::NewWorkflow => workflow::entrypoint(),
        });

    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
