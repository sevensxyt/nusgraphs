use crate::config::{Config, FilterRule};
use crate::errors::GraphError;
use crate::models::{Edge, Graph, ModuleInfo, Node, NodeData, Position};
use crate::storage::{self, Storable};

pub struct GraphTransformer {
    modules: Vec<ModuleInfo>,
}

impl GraphTransformer {
    pub fn new(modules: Vec<ModuleInfo>, config: Config) -> Self {
        let modules = modules
            .into_iter()
            .filter(|module| {
                config.filters.iter().all(|filter| match filter.rule {
                    FilterRule::Include => module.department == filter.target,
                    FilterRule::Exclude => module.department != filter.target,
                })
            })
            .take(config.limit.unwrap_or(usize::MAX))
            .collect();

        Self { modules }
    }

    pub fn build(&self) -> Result<(), GraphError> {
        let graph = if let Some(graph) = storage::read_object::<Graph>() {
            println!("Graph exists at {}", Graph::path());
            graph
        } else {
            let graph = self.transform();
            storage::write_object(&graph)?;
            println!("Graph written to storage at {}", Graph::path());
            graph
        };

        println!(
            "Statistics:\nNodes: {}\nEdges: {}",
            graph.nodes.len(),
            graph.edges.len()
        );

        Ok(())
    }

    pub fn exists() -> bool {
        storage::read_object::<Graph>().is_some()
    }

    pub fn transform(&self) -> Graph {
        Graph {
            nodes: self.create_nodes(),
            edges: self.create_edges(),
        }
    }

    fn create_nodes(&self) -> Vec<Node> {
        self.modules
            .iter()
            .enumerate()
            .map(|(i, module)| Node {
                id: module.module_code.clone(),
                position: self.calculate_node_position(i),
                data: NodeData {
                    label: module.title.clone(),
                    module_code: module.module_code.clone(),
                    title: module.title.clone(),
                    department: module.department.clone(),
                    description: module.description.clone(),
                    module_credit: module.module_credit.clone(),
                    acad_year: module.acad_year.clone(),
                },
                node_type: "module".to_string(),
                node_type_2: "module".to_string(),
            })
            .collect()
    }

    fn create_edges(&self) -> Vec<Edge> {
        self.modules
            .iter()
            .flat_map(|module| {
                module.fulfill_requirements.iter().map(|target| Edge {
                    id: format!("{}-{}", module.module_code, target),
                    source: module.module_code.clone(),
                    target: target.clone(),
                    edge_type: "fulfill_requirement".to_string(),
                    animated: true,
                    label: "fulfill_requirement".to_string(),
                })
            })
            .collect()
    }

    fn calculate_node_position(&self, index: usize) -> Position {
        Position { x: 0.0, y: 0.0 }
    }
}
