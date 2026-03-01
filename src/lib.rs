//! RUST-SA -- Core types
//! All analysis findings are normalized to this schema
//! before framework mapping and report generation

use serde::{Deserialize, Serialize};

/// Severity of a finding, aligned to DoD/CVSS conventions
#[derive(Debug, CLone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
  Critical,
  High,
  Medium,
  Low,
  Informational,
}

/// Confidence that this finding is a true positive
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
  High,
  Medium,
  Low,
}

/// Which tool generated the underlying raw finding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceTool {
  Rudra, 
  CargoGeiger,
  Semgrep,
  Custom,
}

/// File location for a finding 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileLocation {
  pub file: String,
  pub line: u32,
  pub column: Option<u32>,
  pub snippet: Option<String>,
}

/// a reference to an external compliance framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkRef {
  pub framework: String,   // example: NIST SP 800-53
  pub id: String,   //example: SI-10
  pub url: Option<String>,   //link to authoritative source
}

/// Canonical finding type -- all tools normalize to this
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
  /// Unique, stable rule ID (example: RUSTSA-SEC-001)
pub rule_id: String,
pub severity: Severity,
pub confidence: Confidence,
pub source_tool: SourceTool,
pub location: FileLocation,
pub title: String,
pub description: String,
pub remediation: String,
pub framework_refs: Vec<FrameworkRef>,
  /// Trie if suppressed by config or inline annotation
pub suppressed: bool,
  /// Required justification when suppressed = true
pub suppression_reason: Option<String>,
}
