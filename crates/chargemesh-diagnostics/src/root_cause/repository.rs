//! Pattern repository

use super::*;

#[derive(Debug, Clone)]
pub struct RootCausePattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub symptoms: Vec<String>,
    pub causes: Vec<PossibleCause>,
    pub confidence_weight: f64,
}

pub struct PatternRepository {
    patterns: Vec<RootCausePattern>,
}

impl PatternRepository {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn register(&mut self, pattern: RootCausePattern) {
        self.patterns.push(pattern);
    }

    pub fn find_matching(&self, symptoms: &HashSet<String>) -> Vec<&RootCausePattern> {
        let mut matches = Vec::new();

        for pattern in &self.patterns {
            let matching_symptoms: Vec<&String> = pattern.symptoms.iter()
                .filter(|s| symptoms.contains(s.clone()))
                .collect();

            if !matching_symptoms.is_empty() {
                matches.push(pattern);
            }
        }

        matches.sort_by(|a, b| {
            let a_count = a.symptoms.iter().filter(|s| symptoms.contains(s.clone())).count();
            let b_count = b.symptoms.iter().filter(|s| symptoms.contains(s.clone())).count();
            b_count.cmp(&a_count)
        });

        matches
    }

    pub fn get_pattern(&self, id: &str) -> Option<&RootCausePattern> {
        self.patterns.iter().find(|p| p.id == id)
    }
}