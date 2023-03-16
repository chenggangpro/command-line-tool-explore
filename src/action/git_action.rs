#[allow(dead_code)]
#[allow(unused)]
use cmd_lib::{run_cmd, run_fun};
use regex::Regex;
use version_compare::{compare_to, Cmp};

pub const MIN_GIT_VERSION: &str = "2.23.0";

pub struct GitAction {}

impl GitAction {
    pub fn get_git_version() -> Option<String> {
        let git_version_result = run_fun!(git --version);
        let result = match git_version_result {
            Ok(result) => String::from(result),
            Err(e) => panic!("Get git version error : {}", e),
        };
        if !result.is_empty() {
            println!("Current git version : {}", result);
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

    pub fn check_git_version(current_git_version: String) -> bool {
        compare_to(current_git_version, MIN_GIT_VERSION, Cmp::Ge).unwrap_or(false)
    }

    pub fn switch_branch(branch_name: String) {
        println!("[GIT]Switch to branch : {}", branch_name);
        run_cmd!("git switch" + branch_name)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn checkout_branch(branch_name: String) {
        println!("[GIT]Checkout branch : {}", branch_name);
        let name = branch_name;
        run_cmd!(git checkout $name)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn are_there_anything_changed() -> bool {
        println!("[GIT]Check whether there are anything changed in current branch");
        let git_status_result = run_fun!(git status --porcelain --untracked-files=no);
        let result = match git_status_result {
            Ok(result) => String::from(result),
            Err(e) => panic!("Get git version error : {}", e),
        };
        !result.is_empty()
    }

    pub fn create_new_branch_from_current(branch_name: String) {
        println!("[GIT]Create a new branch : {}", branch_name);
        let name = branch_name;
        run_cmd!(git branch $name)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn merge_to_current(source_branch_name: String) {
        println!(
            "[GIT]Merge branch to current,Branch name : {}",
            source_branch_name
        );
        let name = source_branch_name;
        run_cmd!(git merge $name)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn fetch() {
        println!("[GIT]Fetch from remote");
        run_cmd!(git fetch)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn pull() {
        println!("[GIT]Pull from remote");
        run_cmd!(git pull)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn push_branch() {
        println!("[GIT]Push to remote");
        run_cmd!(git push)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn push_tags() {
        println!("[GIT]Push tags to remote");
        run_cmd!(git push --tags)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn is_branch_exists(branch_name: String, is_remote: bool) -> bool {
        let is_remote_display = if is_remote { "remote" } else { "local" };
        println!(
            "[GIT]Check if {} branch exists : {}",
            is_remote_display, branch_name
        );
        let branch_to_verify: String = if is_remote {
            String::from("origin/".to_owned() + branch_name.as_str())
        } else {
            branch_name
        };
        let verify_result = run_fun!(git rev-parse --verify $branch_to_verify);
        let result = match verify_result {
            Ok(result) => String::from(result),
            Err(e) => panic!("Check branch exists error : {}", e),
        };
        !result.starts_with("fatal:")
    }

    pub fn push_new_branch_to_remote(branch_name: String) {
        print!("[GIT]Push new branch to remote : {}", branch_name);
        let name = branch_name;
        run_cmd!(git push --set-upstream origin $name)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn commit(message: String) {
        println!("[GIT]Commit with message");
        run_cmd!(git add ./)
            .map_err(|err| println!("{:?}", err))
            .ok();
        let message_data = message;
        run_cmd!(git commit -m $message_data)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn tag(tag_name: String) {
        println!("[GIT]Create new tag : {}", tag_name);
        let tag = tag_name;
        run_cmd!(git tag $tag)
            .map_err(|err| println!("{:?}", err))
            .ok();
    }

    pub fn get_last_tag_name() -> Option<String> {
        println!("[GIT]Get latest tag name");
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
}
