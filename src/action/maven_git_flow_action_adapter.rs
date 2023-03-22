use crate::action::git_flow_action_adapter::GitFlowActionAdapter;
use crate::execution::maven_execution::MavenExecution;
use crate::support::enums::ProjectType;

pub struct MavenGitFlowActionAdapter {}

impl GitFlowActionAdapter for MavenGitFlowActionAdapter {

    fn current_project_type(&self) -> ProjectType {
        ProjectType::Maven
    }

    fn verify_project(&self) {
        MavenExecution::check_maven_command_exist();
        MavenExecution::verify_project();
    }

    fn get_current_project_version(&self) -> Option<String> {
        MavenExecution::get_current_project_version()
    }

    fn modify_new_version(&self, new_version: &String) {
        MavenExecution::update_version_to(new_version)
    }
}