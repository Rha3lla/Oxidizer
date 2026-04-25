use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    Error, 
    Warning, 
    Note,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Confidence {
    High, 
    Medium, 
    Low,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FindingSource {
    CustomRule, 
    Rudra, 
    CargoGeiger, 
    Semgrep, 
    CargoAudit,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub file: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    pub title: String,
    pub message: String, 
    pub severity: Severity,
    pub confidence: Confidence,
    pub source: FindingSource,
    pub location: Location,
    pub cwe: Option<Vec<String>>,
    pub tags: Vec<String>,
}

impl Finding {
    pub fn fixture() -> Self {
        Finding {
        rule_id: String::from("HA-RUST-0001"),
        title: String::from("Example Finding"),
        message: String::from("This is an example finding for testing purposes."),
        severity: Severity::Error,
        confidence: Confidence::High,
        source: FindingSource::CustomRule,
        location: Location {
            file: String::from("src/main.rs"),
            line: Some(10),
            column: Some(5),
            snippet: Some(String::from("let x = 5;")),
        },
        cwe: None,
        tags: Vec::new(),
        }
    }
} 