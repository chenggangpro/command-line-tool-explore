use std::process::abort;

use cmd_lib::{run_cmd, run_fun};
use colour::{e_blue_ln, e_red_ln};

pub struct MavenExecution {}

impl MavenExecution {
    pub fn update_version_to(version: &String) {
        e_blue_ln!("[MAVEN]Update project's version to new version : {}",version);
        run_cmd!(mvn versions:set -DnewVersion=$version -DgenerateBackupPoms=false)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn check_maven_command_exist() {
        e_blue_ln!("[Verify]Check whether maven is present ...");
        let result = run_fun!(mvn --version);
        let maven_version = match result {
            Ok(output) => Some(String::from(output)),
            Err(_err) => None,
        };
        if maven_version.is_none() {
            e_red_ln!("Command <mvn> does not exist");
            abort();
        }
        e_blue_ln!("Current maven version info : \n {}",maven_version.unwrap())
    }

    pub fn verify_project() {
        e_blue_ln!("[MAVEN]Verify current maven project");
        run_cmd!(mvn clean package -DskipTests -U)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
        run_cmd!(mvn clean)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn update_property_version(property_name: String, property_version: String) {
        e_blue_ln!("[MAVEN]Update property's version ,property's name : {} , new property value : {}",property_name,property_version);
        run_cmd!(mvn versions:set-property -Dproperty=$property_name -DnewVersion=$property_version -DgenerateBackupPoms=false)
            .map_err(|err| {
                e_red_ln!("{:?}", err);
                abort();
            })
            .ok();
    }

    pub fn get_current_project_version() -> Option<String> {
        e_blue_ln!("[MAVEN]Get current project's version");
        let result = run_fun!(mvn -q -Dexec.executable=echo -Dexec.args="$(project.version)" --non-recursive exec:exec);
        match result {
            Ok(output) => Some(String::from(output)),
            Err(_err) => None,
        }
    }
}