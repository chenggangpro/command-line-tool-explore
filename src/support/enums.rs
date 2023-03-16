use std::fmt;
use std::str::FromStr;

use inquire::{CustomUserError, InquireError};

#[derive(Debug)]
pub enum ProjectType {
    // Maven project
    Maven,
    //Webpack project
    Webpack,
}

impl FromStr for ProjectType {
    type Err = InquireError;

    fn from_str(input: &str) -> Result<ProjectType, InquireError> {
        match input {
            "Maven" => Ok(ProjectType::Maven),
            "Webpack" => Ok(ProjectType::Webpack),
            _ => Err(InquireError::Custom(CustomUserError::from(
                "ProjectType not found",
            ))),
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum FlowType {
    //The Feature flow
    Feature,

    //The Hotfix flow
    Hotfix,

    //The Release flow
    Release,
}

impl FromStr for FlowType {
    type Err = InquireError;

    fn from_str(input: &str) -> Result<FlowType, InquireError> {
        match input {
            "Feature" => Ok(FlowType::Feature),
            "Hotfix" => Ok(FlowType::Hotfix),
            "Release" => Ok(FlowType::Release),
            _ => Err(InquireError::Custom(CustomUserError::from(
                "FlowType not found",
            ))),
        }
    }
}

impl fmt::Display for FlowType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
pub enum ReleaseType {
    // none
    None,

    //Release from the Hotfix branch
    Hotfix,

    //Release from the Test branch
    Test,

    //Release from a Specific branch
    Specific,
}

impl FromStr for ReleaseType {
    type Err = InquireError;

    fn from_str(input: &str) -> Result<ReleaseType, InquireError> {
        match input {
            "Hotfix" => Ok(ReleaseType::Hotfix),
            "Test" => Ok(ReleaseType::Test),
            "Specific" => Ok(ReleaseType::Specific),
            _ => Err(InquireError::Custom(CustomUserError::from(
                "ReleaseType not found",
            ))),
        }
    }
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
