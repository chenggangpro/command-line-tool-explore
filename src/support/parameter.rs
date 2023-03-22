use colour::e_blue_ln;
use tabled::{Style, Table, Tabled};

use crate::{FlowType, ReleaseType};
use crate::action::git_flow_action_adapter::GitFlowActionAdapter;
use crate::action::maven_git_flow_action_adapter::MavenGitFlowActionAdapter;
use crate::action::webpack_git_flow_action_adapter::WebpackGitFlowActionAdapter;
use crate::support::enums::ProjectType;

pub struct GitFlowParameter {
    pub project_type: ProjectType,
    pub flow_type: FlowType,
    pub release_type: Option<ReleaseType>,
    pub specific_branch_name: Option<String>,
    pub need_push: bool,
    pub need_push_tag: bool,
}

impl GitFlowParameter {
    pub fn new(project_type: ProjectType, flow_type: FlowType) -> GitFlowParameter {
        GitFlowParameter {
            project_type,
            flow_type,
            release_type: None,
            specific_branch_name: None,
            need_push: false,
            need_push_tag: false,
        }
    }

    pub fn set_release_type(&mut self, release_type: ReleaseType) -> &mut GitFlowParameter {
        self.release_type = Some(release_type);
        self
    }

    pub fn set_specific_release_branch_name(&mut self, specific_release_branch_name: String) -> &mut GitFlowParameter {
        self.specific_branch_name = Some(specific_release_branch_name);
        self
    }

    pub fn set_need_push(&mut self, need_push: bool) -> &mut GitFlowParameter {
        self.need_push = need_push;
        self
    }

    pub fn set_need_push_tag(&mut self, need_push_tag: bool) -> &mut GitFlowParameter {
        self.need_push_tag = need_push_tag;
        self
    }

    pub fn get_git_flow_action_adapter(&mut self) -> Box<dyn GitFlowActionAdapter> {
        if ProjectType::Maven.eq(&self.project_type) {
            Box::new(MavenGitFlowActionAdapter {})
        } else {
            Box::new(WebpackGitFlowActionAdapter {})
        }
    }

    pub fn print_parameters(&mut self) {
        let mut print_contents = Vec::new();
        print_contents.push(PrintContent {
            option: String::from("项目类型"),
            select_value: String::from(self.project_type.to_string()),
        });
        print_contents.push(PrintContent {
            option: String::from("流程类型"),
            select_value: String::from(self.flow_type.to_string()),
        });
        if self.release_type.is_some() {
            print_contents.push(PrintContent {
                option: String::from("Release类型"),
                select_value: String::from(self.release_type.as_mut().unwrap().to_string()),
            });
        }
        if self.specific_branch_name.is_some() {
            print_contents.push(PrintContent {
                option: String::from("指定的Release分支"),
                select_value: String::from(self.specific_branch_name.as_mut().unwrap().to_string()),
            });
        }
        print_contents.push(PrintContent {
            option: String::from("是否推送分支到远端"),
            select_value: String::from(self.need_push.to_string()),
        });
        print_contents.push(PrintContent {
            option: String::from("是否推送Tag到远端"),
            select_value: String::from(self.need_push_tag.to_string()),
        });
        let table = Table::new(print_contents)
            .with(Style::modern())
            .to_string();
        e_blue_ln!("{}",table);
    }
}

#[derive(Tabled)]
struct PrintContent {
    option: String,
    select_value: String,
}
