use serde_json::Value;
use crate::findings::{Finding, Severity};
use std::collections::HashMap;


#[allow(dead_code)]
fn severity_to_level(severity: &Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    }
}


pub fn emit_sarif(findings: &[Finding]) -> Result<String, serde_json::Error> {
    // Step 1: collect one representative finding per unique rule_id
    let mut unique_rules: HashMap<&str, &Finding> = HashMap::new();
    for finding in findings {
        unique_rules.entry(&finding.rule_id).or_insert(finding);
    }

    // Step 2: build the rules array from the deduplicated map
    let rules: Vec<Value> = unique_rules
        .values()
        .map(|f| {
            let mut props = serde_json::json!({
                "tags": f.tags,
            });
            if let Some(cwe) = &f.cwe {
                props["cwe"] = serde_json::json!(cwe);
            }
            serde_json::json!({
                "id": f.rule_id,
                "name": f.title,
                "shortDescription": { "text": f.title },
                "properties": props
            })
        })
        .collect();

    // Step 3: build the results array from all findings
    let results: Vec<Value> = findings
        .iter()
        .map(|f| {
            // Build the region object only if we have at least a line number
            let physical_location = if let Some(line) = f.location.line {
                let mut region = serde_json::json!({ "startLine": line });
                if let Some(col) = f.location.column {
                    region["startColumn"] = serde_json::json!(col);
                }
                serde_json::json!({
                    "artifactLocation": {
                        "uri": f.location.file,
                        "uriBaseId": "%SRCROOT%"
                    },
                    "region": region
                })
            } else {
                serde_json::json!({
                    "artifactLocation": {
                        "uri": f.location.file,
                        "uriBaseId": "%SRCROOT%"
                    }
                })
            };

            serde_json::json!({
                "ruleId": f.rule_id,
                "message": { "text": f.message },
                "level": severity_to_level(&f.severity),
                "locations": [{ "physicalLocation": physical_location }]
            })
        })
        .collect();

    // Step 4: assemble the root SARIF object
    let sarif = serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "oxidizer",
                    "version": env!("CARGO_PKG_VERSION"),
                    "informationUri": "https://github.com/Rha3lla/Oxidizer",
                    "rules": rules
                }
            },
            "results": results
        }]
    });
    serde_json::to_string_pretty(&sarif)
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_findings() {
        // 1. call emit_sarif with empty slice
        let output = emit_sarif(&[]).unwrap();
        // 2. parse the string back to Value
        let value: serde_json::Value =serde_json::from_str(&output).unwrap(); // ensure it's valid JSON
        // 3. assert results == []
        assert!(value["runs"][0]["results"].as_array().unwrap().is_empty());
        // 4. assert rules == []
            assert!(value["runs"][0]["tool"]["driver"]["rules"].as_array().unwrap().is_empty());
    }
}

#[test]
fn test_single_finding(){
    // 1. create a finding
    let test_finding = Finding::fixture();
    // 2. call emit_sarif with a one-element slice
    let output = emit_sarif(&[test_finding]).unwrap();
    // 3. parse the string back to Value
    let value: serde_json::Value =serde_json::from_str(&output).unwrap(); // ensure it's valid JSON
    // 4. assert results has length 1
    assert_eq!(value["runs"][0]["results"].as_array().unwrap().len(), 1);
    // 5. assert rules has length 1
    assert_eq!(value["runs"][0]["tool"]["driver"]["rules"].as_array().unwrap().len(), 1);
    // 6. assert ruleId, level, startLine, startColumn
    let result = &value["runs"][0]["results"][0];
    assert_eq!(result["ruleId"], "HA-RUST-0001");
    assert_eq!(result["level"], "error");
    assert_eq!(result["locations"][0]["physicalLocation"]["region"]["startLine"], 10);
    assert_eq!(result["locations"][0]["physicalLocation"]["region"]["startColumn"], 5);
}

#[test]
fn test_multiple_findings_same_rule() {
    // 1. create two findings with the same rule_id
    let mut finding1 = Finding::fixture();
    finding1.rule_id = "DUPLICATE_RULE".to_string();
    let mut finding2 = Finding::fixture();
    finding2.rule_id = "DUPLICATE_RULE".to_string();
    finding2.location.line = Some(20); // different line to distinguish them

    // 2. call emit_sarif with both findings
    let output = emit_sarif(&[finding1, finding2]).unwrap();
    // 3. parse the string back to Value
    let value: serde_json::Value =serde_json::from_str(&output).unwrap(); // ensure it's valid JSON
    // 4. assert rules has length 1 (deduplication)
    assert_eq!(value["runs"][0]["tool"]["driver"]["rules"].as_array().unwrap().len(), 1);
    // 5. assert results has length 2 (both findings should be present)
    assert_eq!(value["runs"][0]["results"].as_array().unwrap().len(), 2);
}

#[test]
fn test_severity_mapping() {
    // 1. create three findings, each with a different severity and unique rule_id
    let mut error_finding = Finding::fixture();
    error_finding.rule_id = "SEV-ERROR".to_string();
    error_finding.severity = Severity::Error;
    let mut warning_finding = Finding::fixture();
    warning_finding.rule_id = "SEV-WARNING".to_string();
    warning_finding.severity = Severity::Warning;
    let mut note_finding = Finding::fixture();
    note_finding.rule_id = "SEV-NOTE".to_string();
    note_finding.severity = Severity::Note;

    // 2. call emit_sarif with all three as a slice
    let output = emit_sarif(&[error_finding, warning_finding, note_finding]).unwrap();

    // 3. parse the string back to JSON
    let value: serde_json::Value =serde_json::from_str(&output).unwrap(); // ensure it's valid JSON
    // 4. grab the results array
    let results = value["runs"][0]["results"].as_array().unwrap();
    // 5. for each result, match its ruleId to the expected level:
    //    - "SEV-ERROR" → "error"
    //    - "SEV-WARNING" → "warning"
    //    - "SEV-NOTE" → "note"
    for result in results {
        let rule_id = result["ruleId"].as_str().unwrap();
        let level = result["level"].as_str().unwrap();
        match rule_id {
            "SEV-ERROR" => assert_eq!(level, "error"),
            "SEV-WARNING" => assert_eq!(level, "warning"),
            "SEV-NOTE" => assert_eq!(level, "note"),
            _ => panic!("Unexpected ruleId: {}", rule_id),
        }
    }
}