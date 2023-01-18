use std::fmt::Display;
use std::fs::File;
use std::io::Write;

use convert_case::{Case, Casing};
use inquire::{Confirm, Select, Text};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::ApplicationError;

mod res;

#[derive(Serialize)]
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
    wf_type: WorkflowType,
}

impl Context {
    pub fn new(
        context: String,
        entity: String,
        workflow: String,
        is_private: bool,
        wf_type: WorkflowType,
    ) -> Self {
        Self {
            event: match wf_type {
                WorkflowType::Create => format!("{}Created", entity.to_case(Case::Pascal)),
                WorkflowType::Edit => format!("{}Updated", entity.to_case(Case::Pascal)),
                WorkflowType::Delete => format!("{}Deleted", entity.to_case(Case::Pascal)),
            },
            command: match wf_type {
                WorkflowType::Create => format!("Create{}", entity.to_case(Case::Pascal)),
                WorkflowType::Edit => format!("Update{}", entity.to_case(Case::Pascal)),
                WorkflowType::Delete => format!("Delete{}", entity.to_case(Case::Pascal)),
            },
            context: context.clone().to_case(Case::Snake),
            entity_snake_case: entity.to_case(Case::Snake),
            entity_title_case: entity.to_case(Case::Pascal),
            workflow: workflow.to_case(Case::Snake),
            is_private: match is_private {
                true => "True".to_string(),
                false => "False".to_string(),
            },
            repository: format!("{}Repository", context.to_case(Case::Pascal)),
            wf_type: wf_type,
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

fn build_template(context: Context) -> Result<(String, Context), ApplicationError> {
    let mut tt = TinyTemplate::new();
    tt.add_template("CreateWorkflow", res::CREATE_WORKFLOW)
        .unwrap();
    tt.add_template("UpdateWorkflow", res::UPDATE_WORKFLOW)
        .unwrap();

    let render_result = match context.wf_type {
        WorkflowType::Create => tt.render("CreateWorkflow", &context),
        WorkflowType::Edit => tt.render("UpdateWorkflow", &context),
        WorkflowType::Delete => panic!(),
    };

    render_result
        .map(|content| (content, context))
        .map_err(|_| "Failed to render template")
}

fn build_context(wft: WorkflowType) -> Result<Context, ApplicationError> {
    let context = Context::new(
        Text::new("Enter context name").prompt().unwrap(),
        Text::new("Enter entity name").prompt().unwrap(),
        Text::new("Enter workflow name").prompt().unwrap(),
        Confirm::new("Is workflow private?")
            .with_default(false)
            .with_placeholder("No")
            .prompt()
            .unwrap(),
        wft,
    );

    Ok(context)
}

fn write_template(data: (String, Context)) -> Result<(), ApplicationError> {
    let (template, context) = data;

    let mut file = File::create(format!("{}.py", context.workflow)).unwrap();
    file.write_all(template.as_bytes()).unwrap();

    println!("Created!");

    Ok(())
}

pub fn entrypoint() -> Result<(), ApplicationError> {
    choose_type()
        .and_then(build_context)
        .and_then(build_template)
        .and_then(write_template)
}
