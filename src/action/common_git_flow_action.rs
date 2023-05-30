use std::process::abort;

use chrono::Utc;
use colour::{e_dark_yellow_ln, e_green_ln, e_red_ln};

use crate::action::git_flow_action_adapter::GitFlowActionAdapter;
use crate::flow::git_flow::GitFlow;
use crate::GitExecution;
use crate::support::enums::ProjectType;
use crate::support::util::Util;

pub struct CommonGitFlowAction {
    pub git_flow_action_adapter: Box<dyn GitFlowActionAdapter>,

}

impl CommonGitFlowAction {

    pub fn new(git_flow_action_adapter: Box<dyn GitFlowActionAdapter>) -> Box<CommonGitFlowAction> {
        Box::new(CommonGitFlowAction{
            git_flow_action_adapter
        })
    }
}

impl GitFlow for CommonGitFlowAction {
    fn project_type(&self) -> ProjectType {
        self.git_flow_action_adapter.current_project_type()
    }

    fn verify_project(&self) {
        self.git_flow_action_adapter.verify_project()
    }

    fn do_feature(&self) {
        GitExecution::fetch();
        GitExecution::switch_branch(&String::from(Self::DEVELOP_BRANCH));
        GitExecution::pull();
        let new_version = &self.git_flow_action_adapter.get_current_project_version()
            .unwrap_or(String::from("1.0.0-SNAPSHOT"));
        let option_actual_new_version = Util::substring_after(new_version, "v");
        if option_actual_new_version.is_none() {
            e_red_ln!("Can not extract new version number for :{}",new_version);
            abort();
        }
        let actual_new_version = &option_actual_new_version.unwrap();
        let option_version_number = Util::substring_before(actual_new_version, "-SNAPSHOT");
        if option_version_number.is_none() {
            e_red_ln!("Can not extract new version number for :{}",actual_new_version);
            abort();
        }
        let version_number = option_version_number.unwrap();
        let new_branch_name = &(Self::FEATURE_BRANCH.to_owned() + "/" + version_number.as_str());
        let remote_branch_exists = GitExecution::is_branch_exists(new_branch_name, true);
        if remote_branch_exists {
            GitExecution::fetch();
            GitExecution::switch_branch(new_branch_name);
            GitExecution::pull();
            e_green_ln!("Feature branch exists ,Branch name : {}",new_branch_name);
            return;
        }
        let local_branch_exists = GitExecution::is_branch_exists(new_branch_name, false);
        if local_branch_exists {
            GitExecution::switch_branch(new_branch_name);
            e_green_ln!("Feature branch exists ,Branch name : {}",new_branch_name);
            return;
        }
        GitExecution::create_new_branch_from_current(new_branch_name);
        GitExecution::checkout_branch(new_branch_name);
        self.git_flow_action_adapter.modify_new_version(new_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit(String::from("new feature: ").to_owned() + version_number.to_string().as_str())
        }
        e_green_ln!("Feature flow execution completed,Branch name : {}",new_branch_name);
    }

    fn do_hotfix(&self) {
        GitExecution::fetch();
        GitExecution::switch_branch(&String::from(Self::MASTER_BRANCH));
        GitExecution::pull();
        let option_last_tag_name = GitExecution::get_last_tag_name();
        if option_last_tag_name.is_none() {
            e_red_ln!("There is no tag exists in master branch,Hotfix branch can not be created");
            abort();
        }
        let last_tag_name = option_last_tag_name.unwrap();
        let substring = Util::substring_after(&last_tag_name, "v");
        if substring.is_none() {
            e_red_ln!("Can not extract version from tag");
            abort();
        }
        let option_latest_version_number = Util::substring_before(&substring.unwrap(), ".RELEASE");
        if option_latest_version_number.is_none() {
            e_red_ln!("Can not extract latest version number from tag");
            abort();
        }
        let mut version_numbers = option_latest_version_number.unwrap()
            .split(".")
            .map(|item| String::from(item))
            .collect::<Vec<String>>();
        version_numbers[2] = ((&version_numbers[2].parse::<i32>().unwrap()) + 1).to_string();
        let new_version_number = version_numbers.join(".");
        let new_branch_name = Self::HOTFIX_BRANCH.to_owned() + "/" + new_version_number.as_str();
        let remote_branch_exists = GitExecution::is_branch_exists(&new_branch_name, true);
        if remote_branch_exists {
            GitExecution::fetch();
            GitExecution::switch_branch(&new_branch_name);
            GitExecution::pull();
            e_green_ln!("Hotfix branch exists ,Branch name :{}",new_branch_name);
            return;
        }
        let local_branch_exists = GitExecution::is_branch_exists(&new_branch_name, false);
        if local_branch_exists {
            GitExecution::switch_branch(&new_branch_name);
            e_green_ln!("Hotfix branch exists ,Branch name :{}",new_branch_name);
            return;
        }
        GitExecution::create_new_branch_from_current(&new_branch_name);
        GitExecution::checkout_branch(&new_branch_name);
        let new_version = new_version_number.as_str().to_owned() + "-SNAPSHOT";
        self.git_flow_action_adapter.modify_new_version(&new_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit("new hotfix: ".to_owned() + new_version_number.as_str())
        }
        e_green_ln!("Hotfix flow execution completed,Branch name : {}",new_branch_name);
    }

    fn do_release_test(&self) {
        GitExecution::fetch();
        GitExecution::switch_branch(&String::from(Self::MASTER_BRANCH));
        GitExecution::pull();
        let to_release_version_number = GitExecution::get_last_tag_name()
            .and_then(|last_tag_name| {
                Util::substring_after(&last_tag_name, "v")
            })
            .and_then(|sub| {
                Util::substring_before(&sub, ".RELEASE")
            })
            .map(|last_version_number| {
                let mut vec = last_version_number.split(".")
                    .map(|item| String::from(item))
                    .collect::<Vec<String>>();
                vec[1] = ((&vec[1].parse::<i32>().unwrap()) + 1).to_string();
                vec[2] = String::from("0");
                vec.join(".")
            })
            .unwrap_or(String::from("1.0.0"));
        let to_release_branch_name = Self::TEST_BRANCH.to_owned().to_owned() + "/" + to_release_version_number.as_str();
        let remote_branch_exist = GitExecution::is_branch_exists(&to_release_branch_name, true);
        let local_branch_exist = GitExecution::is_branch_exists(&to_release_branch_name, false);
        if remote_branch_exist || local_branch_exist {
            GitExecution::fetch();
            GitExecution::switch_branch(&to_release_branch_name);
            if remote_branch_exist {
                GitExecution::pull();
            }
        } else {
            e_red_ln!("Test branch doesn't exists ,Branch name : {}",to_release_branch_name);
            abort();
        }
        GitExecution::checkout_branch(&String::from(Self::MASTER_BRANCH));
        GitExecution::pull();
        GitExecution::merge_to_current(&to_release_branch_name);
        let release_version = to_release_version_number.to_owned() + ".RELEASE";
        self.git_flow_action_adapter.modify_new_version(&release_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit("release: ".to_owned() + to_release_version_number.as_str());
        }
        let date_str = Utc::now().format("%Y%m%d").to_string();
        let mut new_version_vec = Vec::new();
        new_version_vec.push(release_version.as_str());
        new_version_vec.push(&date_str.as_str());
        let new_tag_name = new_version_vec.join(".");
        let actual_tag_name = String::from("v".to_owned() + new_tag_name.as_str());
        GitExecution::tag(&actual_tag_name);
        let mut next_version_number_vec = to_release_version_number.split(".")
            .map(|item| String::from(item))
            .collect::<Vec<String>>();
        next_version_number_vec[1] = ((&next_version_number_vec[1].parse::<i32>().unwrap()) + 1).to_string();
        let new_feature_version_number = next_version_number_vec.join(".");
        let next_feature_version = new_feature_version_number.to_owned() + "-SNAPSHOT";
        self.git_flow_action_adapter.modify_new_version(&next_feature_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit("back to : ".to_owned() + next_feature_version.as_str());
        }
        GitExecution::switch_branch(&String::from(Self::DEVELOP_BRANCH));
        GitExecution::pull();
        GitExecution::merge_to_current(&String::from(Self::MASTER_BRANCH));
        let next_feature_branch_name = Self::FEATURE_BRANCH.to_owned() + "/" + new_feature_version_number.as_str();
        GitExecution::create_new_branch_from_current(&next_feature_branch_name);
        GitExecution::checkout_branch(&next_feature_branch_name);
        e_green_ln!("Feature release flow execution completed,Release tag name : {} , Next feature branch : {}" ,actual_tag_name,next_feature_branch_name);
    }

    fn do_release_specific(&self, release_source_branch: String) {
        GitExecution::fetch();
        GitExecution::switch_branch(&release_source_branch);
        GitExecution::pull();
        let option_current_project_version = self.git_flow_action_adapter.get_current_project_version();
        if option_current_project_version.is_none() {
            e_red_ln!("Can not get current project's version from branch : {}",release_source_branch);
            abort();
        }
        let current_project_version = option_current_project_version.unwrap();
        let is_snapshot = current_project_version.ends_with("-SNAPSHOT");
        if !is_snapshot {
            e_red_ln!("Current project's version is not a SNAPSHOT version,Version : {}" ,current_project_version);
            abort();
        }
        let option_to_release_version_number = Util::substring_before(&current_project_version, "-SNAPSHOT");
        if option_to_release_version_number.is_none() {
            e_red_ln!("Can not extract to release version number : {}" ,current_project_version);
            abort();
        }
        let to_release_version_number = option_to_release_version_number.unwrap();
        GitExecution::switch_branch(&String::from(Self::MASTER_BRANCH));
        GitExecution::pull();
        GitExecution::merge_to_current(&release_source_branch);
        let release_version = to_release_version_number.as_str().to_owned() + ".RELEASE";
        self.git_flow_action_adapter.modify_new_version(&release_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit("release: ".to_owned() + to_release_version_number.as_str());
        }
        let date_str = Utc::now().format("%Y%m%d").to_string();
        let mut new_version_vec = Vec::new();
        new_version_vec.push(release_version.as_str());
        new_version_vec.push(&date_str.as_str());
        let new_tag_name = new_version_vec.join(".");
        let actual_tag_name = String::from("v".to_owned() + new_tag_name.as_str());
        GitExecution::tag(&actual_tag_name);
        e_dark_yellow_ln!("Release flow of specific branch execution completed,you should merge master into other feature/test branch to keep code as same as mater's updates");
        e_green_ln!("Specific release flow execution completed,Release tag name : {}",actual_tag_name);
    }

    fn do_release_hotfix(&self) {
        GitExecution::fetch();
        GitExecution::switch_branch(&String::from(Self::MASTER_BRANCH));
        GitExecution::pull();
        let option_to_release_version_number = GitExecution::get_last_tag_name()
            .and_then(|last_tag_name| {
                Util::substring_after(&last_tag_name, "v")
            })
            .and_then(|sub| {
                Util::substring_before(&sub, ".RELEASE")
            })
            .map(|last_version_number| {
                let mut vec = last_version_number.split(".")
                    .map(|item| String::from(item))
                    .collect::<Vec<String>>();
                vec[2] = ((&vec[2].parse::<i32>().unwrap()) + 1).to_string();
                vec.join(".")
            });
        if option_to_release_version_number.is_none() {
            e_red_ln!("There is no tag exists in master branch,Hotfix can not release without previous tag");
            abort();
        }
        let to_release_version_number = option_to_release_version_number.unwrap();
        let to_release_branch_name = Self::HOTFIX_BRANCH.to_owned().to_owned() + "/" + to_release_version_number.as_str();
        let remote_branch_exist = GitExecution::is_branch_exists(&to_release_branch_name, true);
        let local_branch_exist = GitExecution::is_branch_exists(&to_release_branch_name, false);
        if remote_branch_exist || local_branch_exist {
            GitExecution::fetch();
            GitExecution::switch_branch(&to_release_branch_name);
            if remote_branch_exist {
                GitExecution::pull();
            }
        } else {
            e_red_ln!("Hotfix branch doesn't exists ,Branch name : {}",to_release_branch_name);
            abort();
        }
        GitExecution::checkout_branch(&String::from(Self::MASTER_BRANCH));
        GitExecution::pull();
        GitExecution::merge_to_current(&to_release_branch_name);
        let release_version = to_release_version_number.to_owned() + ".RELEASE";
        self.git_flow_action_adapter.modify_new_version(&release_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit("release: ".to_owned() + to_release_version_number.as_str());
        }
        let date_str = Utc::now().format("%Y%m%d").to_string();
        let mut new_version_vec = Vec::new();
        new_version_vec.push(release_version.as_str());
        new_version_vec.push(&date_str.as_str());
        let new_tag_name = new_version_vec.join(".");
        let actual_tag_name = String::from("v".to_owned() + new_tag_name.as_str());
        GitExecution::tag(&actual_tag_name);
        let mut next_version_number_vec = to_release_version_number.split(".")
            .map(|item| String::from(item))
            .collect::<Vec<String>>();
        next_version_number_vec[1] = ((&next_version_number_vec[1].parse::<i32>().unwrap()) + 1).to_string();
        next_version_number_vec[2] = String::from("0");
        let new_feature_version_number = next_version_number_vec.join(".");
        let next_feature_version = new_feature_version_number.to_owned() + "-SNAPSHOT";
        self.git_flow_action_adapter.modify_new_version(&next_feature_version);
        let anything_changed = GitExecution::are_there_anything_changed();
        if anything_changed {
            GitExecution::commit("back to : ".to_owned() + next_feature_version.as_str());
        }
        GitExecution::switch_branch(&String::from(Self::DEVELOP_BRANCH));
        GitExecution::pull();
        GitExecution::merge_to_current(&String::from(Self::MASTER_BRANCH));
        e_green_ln!("Hotfix release flow execution completed,Release tag name : {}" ,actual_tag_name);
    }
}