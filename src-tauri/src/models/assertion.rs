use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source of the value to assert against.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "value")]
pub enum AssertionSource {
    StatusCode,
    ResponseTime,
    Header(String),
    JsonPath(String),
    BodyContains(String),
}

/// Comparison operator for assertions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AssertionOperator {
    Equals,
    NotEquals,
    Contains,
    NotContains,
    GreaterThan,
    LessThan,
    Exists,
    NotExists,
    TypeIs,
}

/// A single assertion rule attached to a request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssertionRule {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub source: AssertionSource,
    pub operator: AssertionOperator,
    #[serde(default)]
    pub expected: Option<String>,
    #[serde(default = "default_true")]
    pub is_enabled: bool,
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let rule = AssertionRule {
            id: Uuid::new_v4(),
            source: AssertionSource::StatusCode,
            operator: AssertionOperator::Equals,
            expected: Some("200".to_string()),
            is_enabled: true,
        };
        let json = serde_json::to_string(&rule).unwrap();
        let parsed: AssertionRule = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.expected, Some("200".to_string()));
        assert!(parsed.is_enabled);
    }

    #[test]
    fn serde_json_path_source() {
        let rule = AssertionRule {
            id: Uuid::new_v4(),
            source: AssertionSource::JsonPath("$.data.id".to_string()),
            operator: AssertionOperator::Exists,
            expected: None,
            is_enabled: true,
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("jsonPath"));
        let parsed: AssertionRule = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.source,
            AssertionSource::JsonPath("$.data.id".to_string())
        );
    }

    #[test]
    fn serde_header_source() {
        let rule = AssertionRule {
            id: Uuid::new_v4(),
            source: AssertionSource::Header("content-type".to_string()),
            operator: AssertionOperator::Contains,
            expected: Some("json".to_string()),
            is_enabled: true,
        };
        let json = serde_json::to_string(&rule).unwrap();
        let parsed: AssertionRule = serde_json::from_str(&json).unwrap();
        assert_eq!(
            parsed.source,
            AssertionSource::Header("content-type".to_string())
        );
    }

    #[test]
    fn deserialize_defaults() {
        let json = r#"{"source":{"type":"statusCode"},"operator":"equals"}"#;
        let rule: AssertionRule = serde_json::from_str(json).unwrap();
        assert!(rule.is_enabled);
        assert!(rule.expected.is_none());
    }
}
