use serde::{Deserialize, Serialize};

/// Proxy configuration for HTTP requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    #[serde(default)]
    pub proxy_url: String,
    #[serde(default)]
    pub no_proxy: String,
    #[serde(default)]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let config = ProxyConfig {
            proxy_url: "http://proxy:8080".to_string(),
            no_proxy: "localhost,127.0.0.1".to_string(),
            enabled: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        let parsed: ProxyConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.proxy_url, "http://proxy:8080");
        assert_eq!(parsed.no_proxy, "localhost,127.0.0.1");
        assert!(parsed.enabled);
    }

    #[test]
    fn default_is_disabled() {
        let config = ProxyConfig::default();
        assert!(!config.enabled);
        assert!(config.proxy_url.is_empty());
        assert!(config.no_proxy.is_empty());
    }

    #[test]
    fn deserialize_missing_fields_uses_defaults() {
        let json = r#"{}"#;
        let config: ProxyConfig = serde_json::from_str(json).unwrap();
        assert!(!config.enabled);
        assert!(config.proxy_url.is_empty());
    }
}
