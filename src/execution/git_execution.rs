#[allow(dead_code)]
#[allow(unused)]
use std::process::abort;

use cmd_lib::{run_cmd, run_fun};
use colour::{e_blue_ln, e_red_ln};
use regex::Regex;
use version_compare::{Cmp, compare_to};

pub const MIN_GIT_VERSION: &str = "2.23.0";

pub struct GitExecution {}

impl GitExecution {
    pub fn get_git_version() -> Option<String> {
        let git_version_result = run_fun!(git --version);
        let result = match git_version_result {
            Ok(result) => String::from(result),
            Err(e) => {
                e_red_ln!("Get git version error : {}", e);
                abort();
            }
        };
        if !result.is_empty() {
            e_blue_ln!("Current git version : {}", result);
            let version_number_regex = Regex::new(r"\d*\.\d*\.\d*").unwrap();
            let current_version = version_number_regex
                .find(result.as_str())
                .map(|x| x.as_str())
                .unwrap_or("");
            Some(String::from(current_version))
        } else {
            None
        }
    }

    pub fn check_git_version(current_git_version: &String) -> bool {
        compare_to(current_git_version, MIN_GIT_VERSION, Cmp::Ge).unwrap_or(false)
    }

    pub fn switch_branch(branch_name: &String) {
        e_blue_ln!("[GIT]Switch to branch : {}", branch_name);
        run_cmd!(git switch $branch_name)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn checkout_branch(branch_name: &String) {
        e_blue_ln!("[GIT]Checkout branch : {}", branch_name);
        run_cmd!(git checkout $branch_name)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn are_there_anything_changed() -> bool {
        e_blue_ln!("[GIT]Check whether there are anything changed in current branch");
        let git_status_result = run_fun!(git status --porcelain --untracked-files=no);
        let result = match git_status_result {
            Ok(result) => String::from(result),
            Err(e) => {
                e_red_ln!("Get git version error : {}", e);
                abort();
            }
        };
        !result.is_empty()
    }

    pub fn create_new_branch_from_current(branch_name: &String) {
        e_blue_ln!("[GIT]Create a new branch : {}", branch_name);
        run_cmd!(git branch $branch_name)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn merge_to_current(source_branch_name: &String) {
        e_blue_ln!(
            "[GIT]Merge branch to current,Branch name : {}",
            source_branch_name
        );
        run_cmd!(git merge $source_branch_name)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn fetch() {
        e_blue_ln!("[GIT]Fetch from remote");
        run_cmd!(git fetch)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn pull() {
        e_blue_ln!("[GIT]Pull from remote");
        run_cmd!(git pull)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn push_branch() {
        e_blue_ln!("[GIT]Push to remote");
        run_cmd!(git push)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn push_tags() {
        e_blue_ln!("[GIT]Push tags to remote");
        run_cmd!(git push --tags)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn is_branch_exists(branch_name: &String, is_remote: bool) -> bool {
        let is_remote_display = if is_remote { "remote" } else { "local" };
        e_blue_ln!(
            "[GIT]Check if {} branch exists : {}",
            is_remote_display, branch_name
        );
        let branch_to_verify: String = if is_remote {
            let mut origin = String::from("origin/");
            origin.push_str(branch_name.as_str());
            origin
        } else {
            branch_name.to_string()
        };
        let verify_result = run_fun!(git rev-parse --verify $branch_to_verify);
        let result = match verify_result {
            Ok(result) => String::from(result),
            Err(_) => {
                String::from("")
            }
        };
        !result.starts_with("fatal:")
    }

    pub fn push_new_branch_to_remote(branch_name: &String) {
        print!("[GIT]Push new branch to remote : {}", branch_name);
        run_cmd!(git push --set-upstream origin $branch_name)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn commit(message: String) {
        e_blue_ln!("[GIT]Commit with message");
        run_cmd!(git add ./)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
        run_cmd!(git commit -m $message)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn tag(tag_name: &String) {
        e_blue_ln!("[GIT]Create new tag : {}", tag_name);
        run_cmd!(git tag $tag_name)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn get_last_tag_name() -> Option<String> {
        e_blue_ln!("[GIT]Get latest tag name");
        let git_tag_result = run_fun!(git rev-list --tags --max-count=1);
        let git_tag = match git_tag_result {
            Ok(output) => Some(String::from(output)),
            Err(_err) => None,
        };
        if git_tag.is_none() {
            return None;
        }
        let git_tag_value = git_tag.unwrap();
        let actual_tag = run_fun!(git describe --tags $git_tag_value);
        match actual_tag {
            Ok(output) => Some(String::from(output)),
            Err(_err) => None,
        }
    }

    pub fn list_all_branch() -> Vec<String> {
        let result = run_fun!(git branch | cut -c 3-);
        let option_result = match result {
            Ok(value) => Some(String::from(value)),
            Err(_e) => None
        };
        let option_branch_vec = option_result
            .map(|value| {
                value.split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            });
        if option_branch_vec.is_none() {
            e_red_ln!("未获取到任何本地分支");
            abort();
        }
        option_branch_vec.unwrap()
    }
}
