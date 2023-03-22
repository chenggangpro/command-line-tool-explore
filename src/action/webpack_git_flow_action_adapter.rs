use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::process::abort;

use colour::{e_blue_ln, e_red_ln};

use crate::action::git_flow_action_adapter::GitFlowActionAdapter;
use crate::support::enums::ProjectType;

pub struct WebpackGitFlowActionAdapter {}

impl GitFlowActionAdapter for WebpackGitFlowActionAdapter {
    fn current_project_type(&self) -> ProjectType {
        ProjectType::Webpack
    }

    fn verify_project(&self) {
        e_blue_ln!("[Verify]Verify Webpack project ...");
        let paths = fs::read_dir("./").unwrap();
        let mut package_json_file_path: Option<String> = None;
        for path in paths {
            let temp = path.unwrap().path().display().to_string();
            if temp.eq("package.json") {
                package_json_file_path = Option::Some(temp);
            }
        }
        if package_json_file_path.is_none() {
            e_red_ln!("Current directory does not has a package.json file,please check work dir");
            abort();
        }
    }

    fn get_current_project_version(&self) -> Option<String> {
        e_blue_ln!("[Webpack]Get current project's version");
        let paths = fs::read_dir("./").unwrap();
        let mut package_json_file_path: Option<String> = None;
        for path in paths {
            let temp = path.unwrap().path().display().to_string();
            if temp.eq("package.json") {
                package_json_file_path = Option::Some(temp);
            }
        }
        if package_json_file_path.is_none() {
            return None;
        }
        let f = BufReader::new(File::open("./".to_owned() + package_json_file_path.unwrap().as_str()).unwrap());
        let it = f.lines()
            .map(|line| line.unwrap())
            .filter(|line| line.starts_with("\"version\":"))
            .next();
        if it.is_none() {
            return None;
        }
        let line = it.unwrap();
        let start_bytes = line.find("\"version\": \"");
        let end_bytes = line.find("\",");
        if start_bytes.is_none() || end_bytes.is_none() {
            return None;
        }
        let result = &line[start_bytes.unwrap()..end_bytes.unwrap()];
        Some(String::from(result))
    }

    fn modify_new_version(&self, new_version: &String) {
        e_blue_ln!("[Webpack]Update project's version to new version : {}",new_version);
        let paths = fs::read_dir("./").unwrap();
        let mut package_json_file_path: Option<String> = None;
        for path in paths {
            let temp = path.unwrap().path().display().to_string();
            if temp.eq("package.json") {
                package_json_file_path = Option::Some(temp);
            }
        }
        if package_json_file_path.is_none() {
            e_red_ln!("Current directory does not has a package.json file,please check work dir");
            abort();
        }
        let package_json_file_path: String = "./".to_owned() + package_json_file_path.unwrap().as_str();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(package_json_file_path.clone())
            .expect("package.json doesn't exist or so");
        let lines = BufReader::new(file)
            .lines()
            .map(|line| {
                let string = line.unwrap();
                if string.starts_with("\"version\":") {
                    return "  \"version\": \"".to_owned() + new_version.as_str() + "\",";
                }
                string
            })
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(package_json_file_path.clone().as_str(), lines).expect("Can't write lines to file");
    }
}