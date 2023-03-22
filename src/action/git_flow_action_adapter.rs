use crate::support::enums::ProjectType;

pub trait GitFlowActionAdapter {

    // Current project type
    fn current_project_type(&self) -> ProjectType;

    /// Verify current project
    fn verify_project(&self);

    /// Gets current project version
    fn get_current_project_version(&self) -> Option<String>;

    /// Modify new version
    fn modify_new_version(&self,new_version: &String);
}