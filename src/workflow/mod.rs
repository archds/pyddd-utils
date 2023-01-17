use std::fmt::Display;
use std::fs::File;
use std::io::Write;

use convert_case::{Case, Casing};
use inquire::{Confirm, Select, Text};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::ApplicationError;

mod res;

enum WorkflowType {
    Create,
    Edit,
    Delete,
}

#[derive(Serialize)]
struct Context {
    event: String,
    command: String,
    context: String,
    repository: String,
    entity_snake_case: String,
    entity_title_case: String,
    workflow: String,
    is_private: String,
}

impl Context {
    pub fn new(context: String, entity: String, workflow: String, is_private: bool) -> Self {
        Self {
            event: format!("{}Created", entity.to_case(Case::Pascal)),
            command: format!("Create{}", entity.to_case(Case::Pascal)),
            context: context.clone().to_case(Case::Snake),
            entity_snake_case: entity.to_case(Case::Snake),
            entity_title_case: entity.to_case(Case::Pascal),
            workflow: workflow.to_case(Case::Snake),
            is_private: match is_private {
                true => "True".to_string(),
                false => "False".to_string(),
            },
            repository: format!("{}Repository", context.to_case(Case::Pascal)),
        }
    }
}

impl Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create => write!(f, "Create"),
            Self::Edit => write!(f, "Edit"),
            Self::Delete => write!(f, "Delete"),
        }
    }
}

fn choose_type() -> Result<WorkflowType, ApplicationError> {
    Select::new(
        "Select workflow type",
        vec![
            WorkflowType::Create,
            WorkflowType::Edit,
            WorkflowType::Delete,
        ],
    )
    .prompt()
    .map_err(|_| "Invalid workflow type!")
}

fn build_context(wft: WorkflowType) -> Result<Context, ApplicationError> {
    match wft {
        WorkflowType::Create => {
            let context = Context::new(
                Text::new("Enter context name").prompt().unwrap(),
                Text::new("Enter entity name").prompt().unwrap(),
                Text::new("Enter workflow name").prompt().unwrap(),
                Confirm::new("Is workflow private?")
                    .with_default(false)
                    .with_placeholder("No")
                    .prompt()
                    .unwrap(),
            );

            Ok(context)
        }
        WorkflowType::Edit => panic!(),
        WorkflowType::Delete => panic!(),
    }
}

fn write_template(context: Context) -> Result<(), ApplicationError> {
    let mut tt = TinyTemplate::new();
    tt.add_template("CreateWorkflow", res::CREATE_WORKFLOW)
        .unwrap();

    let template = tt.render("CreateWorkflow", &context).unwrap();

    let mut file = File::create(format!("{}.py", context.workflow)).unwrap();
    file.write_all(template.as_bytes()).unwrap();

    Ok(())
}

pub fn entrypoint() -> Result<(), ApplicationError> {
    choose_type()
        .and_then(build_context)
        .and_then(write_template)
}
