use std::process::abort;

use colour::e_red_ln;

use crate::action::common_git_flow_action::CommonGitFlowAction;
use crate::execution::git_execution::{GitExecution, MIN_GIT_VERSION};
use crate::flow::git_flow::GitFlow;
use crate::support::enums::{FlowType, ReleaseType};
use crate::support::parameter::GitFlowParameter;
use crate::support::prompt::{confirm_execution_prompt, get_branch_name_from_select, get_flow_type_from_prompt, get_release_type_from_prompt};
use crate::support::prompt::get_package_type_from_prompt;

pub mod execution;
pub mod support;
pub mod flow;
pub mod action;

fn main() {
    let current_git_version = GitExecution::get_git_version();
    if current_git_version.is_none() {
        e_red_ln!("未在本机器识别到git命令");
        return;
    }

    let is_valid_git_version =
        GitExecution::check_git_version(&current_git_version.unwrap());
    if !is_valid_git_version {
        e_red_ln!("本机git命令，版本过低，最小版本 :{}", MIN_GIT_VERSION);
        return;
    }
    let result = get_package_type_from_prompt();
    let project_type = match result {
        Ok(choice) => choice,
        Err(_) => {
            e_red_ln!("未识别的项目类型，请重试");
            abort();
        }
    };
    let result = get_flow_type_from_prompt();
    let flow_type = match result {
        Ok(choice) => choice,
        Err(_) => {
            e_red_ln!("未识别的流程类型，请重试");
            abort();
        }
    };
    let mut git_flow_parameter = GitFlowParameter::new(project_type, flow_type);
    if FlowType::Release.eq(&git_flow_parameter.flow_type) {
        let release_type_result = get_release_type_from_prompt();
        let result_type = match release_type_result {
            Ok(choice) => choice,
            Err(_) => {
                e_red_ln!("未识别的Release类型，请重试");
                abort();
            }
        };
        if ReleaseType::Specific.eq(&result_type) {
            let specific_branch_name_result = get_branch_name_from_select();
            let specific_branch_name = match specific_branch_name_result {
                Ok(choice) => choice,
                Err(_) => {
                    e_red_ln!("未识别的要指定的合并分支，请重试");
                    abort();
                }
            };
            git_flow_parameter.set_specific_release_branch_name(specific_branch_name);
        }
        git_flow_parameter.set_release_type(result_type);
    }
    // let push_to_remote_result = select_true_or_false("是否Push分支到远端?");
    // let push_to_remote = match push_to_remote_result {
    //     Ok(choice) => choice,
    //     Err(_) => {
    //         e_red_ln!("未识别的选择项，请重试");
    //         abort();
    //     }
    // };
    // let push_tag_to_remote_result = select_true_or_false("是否Push所有Tag到远端?");
    // let push_tag_to_remote = match push_tag_to_remote_result {
    //     Ok(choice) => choice,
    //     Err(_) => {
    //         e_red_ln!("未识别的选择项，请重试");
    //         abort();
    //     }
    // };
    // git_flow_parameter.set_need_push(push_to_remote);
    // git_flow_parameter.set_need_push_tag(push_tag_to_remote);
    git_flow_parameter.print_parameters();
    let confirm_result = confirm_execution_prompt();
    let confirm = match confirm_result {
        Ok(value) => value,
        Err(_) => false
    };
    if !confirm {
        e_red_ln!("已取消任务。");
        abort();
    }
    let git_flow_action_adapter = git_flow_parameter.get_git_flow_action_adapter();
    let common_git_flow_action = CommonGitFlowAction::new(git_flow_action_adapter);
    if FlowType::Feature.eq(&git_flow_parameter.flow_type) {
        common_git_flow_action.do_feature();
        return;
    }
    if FlowType::Hotfix.eq(&git_flow_parameter.flow_type) {
        common_git_flow_action.do_hotfix();
        return;
    }
    if FlowType::Release.eq(&git_flow_parameter.flow_type) {
        let selected_release_type = git_flow_parameter.release_type.unwrap();
        if ReleaseType::Test.eq(&selected_release_type) {
            common_git_flow_action.do_release_test();
            return;
        }
        if ReleaseType::Hotfix.eq(&selected_release_type) {
            common_git_flow_action.do_release_hotfix();
            return;
        }
        if ReleaseType::Specific.eq(&selected_release_type) {
            common_git_flow_action.do_release_specific(git_flow_parameter.specific_branch_name.unwrap());
            return;
        }
    }
}
