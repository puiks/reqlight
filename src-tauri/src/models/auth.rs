use serde::{Deserialize, Serialize};

/// API Key injection location: header or query parameter.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ApiKeyLocation {
    #[default]
    Header,
    Query,
}

/// Authentication configuration for a request.
///
/// Serialized format (externally tagged, camelCase):
///   AuthConfig::None        → "none"
///   AuthConfig::BearerToken → {"bearerToken": {"token": "..."}}
///   AuthConfig::BasicAuth   → {"basicAuth": {"username": "...", "password": "..."}}
///   AuthConfig::ApiKey      → {"apiKey": {"key": "...", "value": "...", "location": "..."}}
///   AuthConfig::OAuth2      → {"oauth2": {"grantType": "...", ...}}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthConfig {
    #[default]
    None,
    BearerToken {
        token: String,
    },
    BasicAuth {
        username: String,
        password: String,
    },
    ApiKey {
        key: String,
        value: String,
        location: ApiKeyLocation,
    },
    #[serde(rename = "oauth2", rename_all = "camelCase")]
    OAuth2 {
        grant_type: String, // "authorization_code" or "client_credentials"
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        scopes: String,
        access_token: String,
        refresh_token: String,
        token_expiry: Option<String>, // ISO8601
    },
}

#[cfg(test)]
#[path = "auth_tests.rs"]
mod tests;
