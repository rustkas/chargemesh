//! Correlation tracer

use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct CorrelationTracer {
    correlations: Arc<RwLock<Vec<Correlation>>>,
    graph: Arc<RwLock<CorrelationGraph>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorrelationGraph {
    pub nodes: HashMap<String, CorrelationNode>,
    pub edges: Vec<CorrelationEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationNode {
    pub id: String,
    pub node_type: CorrelationSource,
    pub label: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationEdge {
    pub source: String,
    pub target: String,
    pub relationship: RelationshipType,
    pub confidence: f64,
}

impl CorrelationTracer {
    pub fn new() -> Self {
        Self {
            correlations: Arc::new(RwLock::new(Vec::new())),
            graph: Arc::new(RwLock::new(CorrelationGraph::default())),
        }
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }

    pub async fn add_correlation(&self, correlation: Correlation) -> Result<()> {
        let mut correlations = self.correlations.write().await;
        correlations.push(correlation.clone());

        let mut graph = self.graph.write().await;

        let source_id = format!("{}:{}", correlation.source_type, correlation.source_id);
        if !graph.nodes.contains_key(&source_id) {
            graph.nodes.insert(source_id.clone(), CorrelationNode {
                id: source_id.clone(),
                node_type: correlation.source_type,
                label: format!("{:?}", correlation.source_type),
                metadata: HashMap::new(),
            });
        }

        let target_id = format!("{}:{}", correlation.target_type, correlation.target_id);
        if !graph.nodes.contains_key(&target_id) {
            graph.nodes.insert(target_id.clone(), CorrelationNode {
                id: target_id.clone(),
                node_type: correlation.target_type,
                label: format!("{:?}", correlation.target_type),
                metadata: HashMap::new(),
            });
        }

        graph.edges.push(CorrelationEdge {
            source: source_id,
            target: target_id,
            relationship: correlation.relationship,
            confidence: correlation.confidence,
        });

        Ok(())
    }

    pub async fn get_correlations(&self) -> Vec<Correlation> {
        self.correlations.read().await.clone()
    }

    pub async fn get_graph(&self) -> CorrelationGraph {
        self.graph.read().await.clone()
    }

    pub async fn get_summary(&self) -> CorrelationSummary {
        let correlations = self.correlations.read().await;

        let device_to_session = correlations.iter()
            .filter(|c| c.source_type == CorrelationSource::Device && c.target_type == CorrelationTarget::Session)
            .map(|c| c.id.clone())
            .collect();

        let session_to_error = correlations.iter()
            .filter(|c| c.source_type == CorrelationSource::Session && c.target_type == CorrelationTarget::Error)
            .map(|c| c.id.clone())
            .collect();

        let error_to_root_cause = correlations.iter()
            .filter(|c| c.source_type == CorrelationSource::Error && c.target_type == CorrelationTarget::RootCause)
            .map(|c| c.id.clone())
            .collect();

        CorrelationSummary {
            device_to_session,
            session_to_error,
            error_to_root_cause,
            total_correlations: correlations.len(),
        }
    }

    pub async fn trace_path(
        &self,
        start_id: &str,
        target_type: CorrelationTarget,
    ) -> Result<Vec<Correlation>> {
        let graph = self.graph.read().await;
        let mut path = Vec::new();
        let mut visited = std::collections::HashSet::new();

        let start_node = graph.nodes.iter()
            .find(|(_, n)| n.id == start_id || n.id.ends_with(start_id))
            .map(|(id, _)| id.clone());

        if let Some(start) = start_node {
            self.trace_path_recursive(&graph, &start, &target_type, &mut path, &mut visited)?;
        }

        Ok(path)
    }

    fn trace_path_recursive(
        &self,
        graph: &CorrelationGraph,
        current: &str,
        target_type: &CorrelationTarget,
        path: &mut Vec<Correlation>,
        visited: &mut std::collections::HashSet<String>,
    ) -> Result<()> {
        if visited.contains(current) {
            return Ok(());
        }
        visited.insert(current.to_string());

        let edges: Vec<&CorrelationEdge> = graph.edges.iter()
            .filter(|e| e.source == *current)
            .collect();

        for edge in edges {
            if let Some(node) = graph.nodes.get(&edge.target) {
                if node.node_type == *target_type {
                    let correlation = Correlation {
                        id: uuid::Uuid::new_v4().to_string(),
                        source_type: node.node_type,
                        target_type: node.node_type,
                        source_id: edge.source.clone(),
                        target_id: edge.target.clone(),
                        relationship: edge.relationship.clone(),
                        confidence: edge.confidence,
                        evidence: Vec::new(),
                        timestamp: chrono::Utc::now(),
                    };
                    path.push(correlation);
                }

                self.trace_path_recursive(graph, &edge.target, target_type, path, visited)?;
            }
        }

        Ok(())
    }
}

impl Default for CorrelationTracer {
    fn default() -> Self {
        Self::new()
    }
}