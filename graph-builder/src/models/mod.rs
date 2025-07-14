use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    #[serde(rename = "moduleCode")]
    pub module_code: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModuleInfo {
    #[serde(rename = "moduleCode")]
    module_code: String,

    #[serde(rename = "acadYear")]
    acad_year: String,

    title: String,
    description: String,
    department: String,
    faculty: String,

    #[serde(rename = "moduleCredit")]
    module_credit: String,

    #[serde(default)]
    prerequisite: Option<String>,

    #[serde(default)]
    preclusion: Option<String>,

    #[serde(rename = "prereqTree", default)]
    prereq_tree: Option<PrereqTree>,

    #[serde(rename = "fulfillRequirements", default)]
    fulfill_requirements: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PrereqTree {
    Module(String),
    Logic {
        and: Option<Vec<PrereqTree>>,
        or: Option<Vec<PrereqTree>>,
    }
}
