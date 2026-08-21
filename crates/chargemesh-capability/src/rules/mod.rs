//! Rule engine for capability evaluation

mod engine;
mod conditions;
mod actions;

pub use engine::*;
pub use conditions::*;
pub use actions::*;

use super::*;

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub name: String,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub priority: u8,
    pub enabled: bool,
}

impl Rule {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            conditions: Vec::new(),
            actions: Vec::new(),
            priority: 100,
            enabled: true,
        }
    }

    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
        // Sort by priority (lower number = higher priority)
        self.rules.sort_by_key(|r| r.priority);
    }

    pub fn get_rules(&self) -> &[Rule] {
        &self.rules
    }

    pub fn evaluate_rule(
        &self,
        rule: &Rule,
        context: &CapabilityContext,
        capabilities: &CapabilitySet,
    ) -> bool {
        if !rule.enabled {
            return false;
        }

        rule.conditions.iter().all(|condition| {
            condition.evaluate(context, capabilities)
        })
    }

    pub fn apply_action(&self, rule: &Rule, capabilities: &mut CapabilitySet) {
        for action in &rule.actions {
            action.apply(capabilities);
        }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}