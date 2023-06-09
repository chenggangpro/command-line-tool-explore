use std::str::FromStr;

use inquire::{Confirm, InquireError, Select};

use crate::GitExecution;
use crate::support::enums::{FlowType, ProjectType, ReleaseType};

pub fn get_package_type_from_prompt() -> Result<ProjectType, InquireError> {
    let options: Vec<&str> = vec!["Maven", "Webpack"];
    let result = Select::new("请选择当前项目的类型...", options)
        .prompt()
        .and_then(|str| ProjectType::from_str(str));
    return result;
}

pub fn get_flow_type_from_prompt() -> Result<FlowType, InquireError> {
    let options: Vec<&str> = vec!["Feature", "Hotfix", "Release"];
    let result = Select::new("请选择要执行的流程...", options)
        .prompt()
        .and_then(|str| FlowType::from_str(str));
    return result;
}

pub fn get_release_type_from_prompt() -> Result<ReleaseType, InquireError> {
    let options: Vec<&str> = vec!["Hotfix", "Test", "Specific"];
    let result = Select::new("请选择一个Release类型...", options)
        .prompt()
        .and_then(|str| ReleaseType::from_str(str));
    return result;
}

pub fn confirm_execution_prompt() -> Result<bool, InquireError> {
    let confirm_result = Confirm::new("是否确认执行上述操作?")
        .with_default(false)
        // .with_help_message("This data is stored for good reasons")
        .prompt();
    return confirm_result;
}

pub fn get_branch_name_from_select() -> Result<String, InquireError> {
    let branch_vec = GitExecution::list_all_branch();
    Select::new("请选择要特殊Release的分支名称", branch_vec)
        .prompt()
        .map(|str| String::from(str))
}

pub fn select_true_or_false(prompt_content: &str) -> Result<bool, InquireError> {
    let branch_vec = vec![true, false];
    Select::new(prompt_content, branch_vec)
        .prompt()
}