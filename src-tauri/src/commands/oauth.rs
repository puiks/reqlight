use crate::services::oauth;
use crate::SharedHttpClient;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthTokenResult {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: Option<u64>,
}

/// IPC: Exchange client credentials for a token.
#[tauri::command]
pub async fn oauth_client_credentials(
    client: State<'_, SharedHttpClient>,
    token_url: String,
    client_id: String,
    client_secret: String,
    scopes: String,
) -> Result<OAuthTokenResult, String> {
    let resp = oauth::client_credentials_exchange(
        &client.0,
        &token_url,
        &client_id,
        &client_secret,
        &scopes,
    )
    .await?;

    Ok(OAuthTokenResult {
        access_token: resp.access_token,
        refresh_token: resp.refresh_token.unwrap_or_default(),
        expires_in: resp.expires_in,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthCodeFlowParams {
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scopes: String,
}

/// IPC: Start the Authorization Code flow.
/// Opens the system browser and waits for the callback.
#[tauri::command]
pub async fn oauth_authorization_code(
    client: State<'_, SharedHttpClient>,
    params: AuthCodeFlowParams,
) -> Result<OAuthTokenResult, String> {
    // Start local callback server
    let (redirect_uri, code) = {
        // Start server first, then open browser
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| format!("Failed to start callback server: {e}"))?;
        let port = listener
            .local_addr()
            .map_err(|e| format!("Failed to get address: {e}"))?
            .port();
        let redirect_uri = format!("http://127.0.0.1:{port}/callback");

        // Build authorization URL
        let auth_url = build_auth_url(
            &params.auth_url,
            &params.client_id,
            &redirect_uri,
            &params.scopes,
        );

        // Open browser
        open::that(&auth_url).map_err(|e| format!("Failed to open browser: {e}"))?;

        // Wait for callback (120s timeout)
        let accept_future = async {
            let (stream, _) = listener
                .accept()
                .await
                .map_err(|e| format!("Accept error: {e}"))?;

            let mut buf = vec![0u8; 4096];
            stream
                .readable()
                .await
                .map_err(|e| format!("Read error: {e}"))?;
            let n = stream
                .try_read(&mut buf)
                .map_err(|e| format!("Read error: {e}"))?;
            let request_str = String::from_utf8_lossy(&buf[..n]);

            let code = oauth::extract_code_from_request(&request_str)?;

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
                <html><body><h2>Authorization successful!</h2>\
                <p>You can close this tab and return to Reqlight.</p>\
                <script>window.close()</script></body></html>";
            let _ = stream.writable().await;
            let _ = stream.try_write(response.as_bytes());

            Ok::<String, String>(code)
        };

        let code = tokio::time::timeout(std::time::Duration::from_secs(120), accept_future)
            .await
            .map_err(|_| "Authorization timed out (120s). Please try again.".to_string())?
            .map_err(|e: String| e)?;

        (redirect_uri, code)
    };

    // Exchange code for token
    let resp = oauth::authorization_code_exchange(
        &client.0,
        &params.token_url,
        &code,
        &params.client_id,
        &params.client_secret,
        &redirect_uri,
    )
    .await?;

    Ok(OAuthTokenResult {
        access_token: resp.access_token,
        refresh_token: resp.refresh_token.unwrap_or_default(),
        expires_in: resp.expires_in,
    })
}

/// IPC: Refresh an expired token.
#[tauri::command]
pub async fn oauth_refresh_token(
    client: State<'_, SharedHttpClient>,
    token_url: String,
    refresh_token: String,
    client_id: String,
    client_secret: String,
) -> Result<OAuthTokenResult, String> {
    let resp = oauth::refresh_token_exchange(
        &client.0,
        &token_url,
        &refresh_token,
        &client_id,
        &client_secret,
    )
    .await?;

    Ok(OAuthTokenResult {
        access_token: resp.access_token,
        refresh_token: resp.refresh_token.unwrap_or_default(),
        expires_in: resp.expires_in,
    })
}

fn build_auth_url(auth_url: &str, client_id: &str, redirect_uri: &str, scopes: &str) -> String {
    let mut url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}",
        auth_url,
        urlencoding::encode(client_id),
        urlencoding::encode(redirect_uri),
    );
    if !scopes.is_empty() {
        url.push_str(&format!("&scope={}", urlencoding::encode(scopes)));
    }
    url
}
