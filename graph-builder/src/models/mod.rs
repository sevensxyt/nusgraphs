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
    pub module_code: String,

    #[serde(rename = "acadYear")]
    pub acad_year: String,

    pub title: String,
    pub description: String,
    pub department: String,
    pub faculty: String,

    #[serde(rename = "moduleCredit")]
    pub module_credit: String,

    #[serde(default)]
    pub prerequisite: Option<String>,

    #[serde(default)]
    pub preclusion: Option<String>,

    #[serde(rename = "prereqTree", default)]
    pub prereq_tree: Option<PrereqTree>,

    #[serde(rename = "fulfillRequirements", default)]
    pub fulfill_requirements: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PrereqTree {
    Module(String),
    Logic {
        #[serde(skip_serializing_if = "Option::is_none")]
        and: Option<Vec<PrereqTree>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        or: Option<Vec<PrereqTree>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "nOf")]
        n_of: Option<(u32, Vec<PrereqTree>)>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeData {
    pub label: String,

    #[serde(rename = "moduleCode")]
    pub module_code: String,
    pub title: String,
    pub department: String,
    pub description: String,

    #[serde(rename = "moduleCredit")]
    pub module_credit: String,

    #[serde(rename = "acadYear")]
    pub acad_year: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: String,
    pub position: Position,
    pub data: NodeData,
    pub node_type: String,
    #[serde(rename = "nodeType")]
    pub node_type_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub animated: bool,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}
