use crate::support::enums::ProjectType;

pub trait GitFlow {
    /// The master branch name
    const MASTER_BRANCH: &'static str =  "master";
    /// The develop branch name
    const DEVELOP_BRANCH: &'static str = "develop";
    /// The feature branch name
    const FEATURE_BRANCH: &'static str ="feature";
    /// The hotfix branch name
    const HOTFIX_BRANCH: &'static str = "hotfix";
    /// The test branch name
    const TEST_BRANCH: &'static str = "test";
    /// Current ProjectType
    fn project_type(&self) -> ProjectType;
    /// Verify current project
    fn verify_project(&self);
    /// Do feature flow
    fn do_feature(&self);
    /// Do hotfix flow
    fn do_hotfix(&self);
    /// Do release flow from test branch
    fn do_release_test(&self);
    /// Do release flow from specific branch
    fn do_release_specific(&self,release_source_branch: String);
    /// Do release flow from hotfix branch
    fn do_release_hotfix(&self);
}