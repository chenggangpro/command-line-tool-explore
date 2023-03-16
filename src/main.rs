use crate::action::git_action::{GitAction, MIN_GIT_VERSION};
use crate::support::prompt::get_flow_type_from_prompt;
use crate::support::prompt::get_package_type_from_prompt;

pub mod action;
pub mod support;

fn main() {
    let result = get_package_type_from_prompt();
    let project_type = match result {
        Ok(choice) => choice,
        Err(_) => panic!("未识别的项目类型，请重试"),
    };
    let result = get_flow_type_from_prompt();
    let flow_type = match result {
        Ok(choice) => choice,
        Err(_) => panic!("未识别的流程类型，请重试"),
    };
    println!("Project Type :{}", project_type.to_string());
    println!("Flow Type :{}", flow_type.to_string());

    let current_git_version = GitAction::get_git_version();
    if current_git_version.is_none() {
        println!("未在本机器识别到git命令");
        return;
    }
    let is_valid_git_version =
        GitAction::check_git_version(String::from(current_git_version.unwrap()));
    if !is_valid_git_version {
        println!("本机git命令，版本过低，最小版本 :{}", MIN_GIT_VERSION);
        return;
    }
    let last_tag_name = GitAction::get_last_tag_name();
    match last_tag_name {
        Some(value) => println!("最新的Tag名称 : {}", value),
        None => print!("未获取到最新的Tag名称"),
    };
}
