use std::fmt::Display;

use inquire::Select;

mod workflow;

type ApplicationError = &'static str;

pub enum Action {
    NewWorkflow,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewWorkflow => write!(f, "New workflow"),
        }
    }
}

fn main() {
    let options: Vec<Action> = vec![Action::NewWorkflow];

    let result = Select::new("Choose command", options)
        .prompt()
        .map(|cmd| match cmd {
            Action::NewWorkflow => workflow::entrypoint(),
        });

    match result {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
