<script lang="ts">
  import { editorStore } from "../../lib/stores/editor.svelte";
  import type { AuthType, ApiKeyLocation, OAuthGrantType } from "../../lib/types";
  import { oauthClientCredentials, oauthAuthorizationCode, oauthRefreshToken } from "../../lib/commands";

  const authTypes: { value: AuthType; label: string }[] = [
    { value: "none", label: "No Auth" },
    { value: "bearerToken", label: "Bearer Token" },
    { value: "basicAuth", label: "Basic Auth" },
    { value: "apiKey", label: "API Key" },
    { value: "oauth2", label: "OAuth 2.0" },
  ];

  const apiKeyLocations: { value: ApiKeyLocation; label: string }[] = [
    { value: "header", label: "Header" },
    { value: "query", label: "Query Param" },
  ];

  const grantTypes: { value: OAuthGrantType; label: string }[] = [
    { value: "client_credentials", label: "Client Credentials" },
    { value: "authorization_code", label: "Authorization Code" },
  ];

  let showToken = $state(false);
  let showPassword = $state(false);
  let showApiKeyValue = $state(false);
  let showOAuthSecret = $state(false);
  let oauthLoading = $state(false);
  let oauthError = $state<string | null>(null);

  function onAuthTypeChange(e: Event) {
    editorStore.authType = (e.target as HTMLSelectElement).value as AuthType;
    showToken = false;
    showPassword = false;
    showApiKeyValue = false;
    showOAuthSecret = false;
    editorStore.markDirty();
  }

  async function fetchOAuthToken() {
    oauthLoading = true;
    oauthError = null;
    try {
      let result;
      if (editorStore.oauthGrantType === "client_credentials") {
        result = await oauthClientCredentials(
          editorStore.oauthTokenUrl,
          editorStore.oauthClientId,
          editorStore.oauthClientSecret,
          editorStore.oauthScopes,
        );
      } else {
        result = await oauthAuthorizationCode({
          authUrl: editorStore.oauthAuthUrl,
          tokenUrl: editorStore.oauthTokenUrl,
          clientId: editorStore.oauthClientId,
          clientSecret: editorStore.oauthClientSecret,
          scopes: editorStore.oauthScopes,
        });
      }
      editorStore.oauthAccessToken = result.accessToken;
      editorStore.oauthRefreshToken = result.refreshToken;
      if (result.expiresIn) {
        const expiry = new Date(Date.now() + result.expiresIn * 1000);
        editorStore.oauthTokenExpiry = expiry.toISOString();
      }
      editorStore.markDirty();
    } catch (e) {
      oauthError = e instanceof Error ? e.message : String(e);
    } finally {
      oauthLoading = false;
    }
  }

  async function refreshOAuthToken() {
    if (!editorStore.oauthRefreshToken) return;
    oauthLoading = true;
    oauthError = null;
    try {
      const result = await oauthRefreshToken(
        editorStore.oauthTokenUrl,
        editorStore.oauthRefreshToken,
        editorStore.oauthClientId,
        editorStore.oauthClientSecret,
      );
      editorStore.oauthAccessToken = result.accessToken;
      if (result.refreshToken) editorStore.oauthRefreshToken = result.refreshToken;
      if (result.expiresIn) {
        const expiry = new Date(Date.now() + result.expiresIn * 1000);
        editorStore.oauthTokenExpiry = expiry.toISOString();
      }
      editorStore.markDirty();
    } catch (e) {
      oauthError = e instanceof Error ? e.message : String(e);
    } finally {
      oauthLoading = false;
    }
  }
</script>

<div class="auth-editor">
  <div class="auth-type-row">
    <label class="label" for="auth-type">Type</label>
    <select id="auth-type" class="select" value={editorStore.authType} onchange={onAuthTypeChange}>
      {#each authTypes as t}
        <option value={t.value}>{t.label}</option>
      {/each}
    </select>
  </div>

  {#if editorStore.authType === "bearerToken"}
    <div class="auth-fields">
      <label class="label" for="bearer-token">Token</label>
      <div class="secret-field">
        <input
          id="bearer-token"
          type={showToken ? "text" : "password"}
          class="input"
          placeholder="Enter bearer token"
          bind:value={editorStore.bearerToken}
          oninput={() => editorStore.markDirty()}
        />
        <button
          class="eye-btn"
          title={showToken ? "Hide" : "Show"}
          onclick={() => (showToken = !showToken)}
        >{showToken ? "◉" : "○"}</button>
      </div>
    </div>
  {:else if editorStore.authType === "basicAuth"}
    <div class="auth-fields">
      <label class="label" for="basic-user">Username</label>
      <input
        id="basic-user"
        type="text"
        class="input"
        placeholder="Username"
        bind:value={editorStore.basicUsername}
        oninput={() => editorStore.markDirty()}
      />
      <label class="label" for="basic-pass">Password</label>
      <div class="secret-field">
        <input
          id="basic-pass"
          type={showPassword ? "text" : "password"}
          class="input"
          placeholder="Password"
          bind:value={editorStore.basicPassword}
          oninput={() => editorStore.markDirty()}
        />
        <button
          class="eye-btn"
          title={showPassword ? "Hide" : "Show"}
          onclick={() => (showPassword = !showPassword)}
        >{showPassword ? "◉" : "○"}</button>
      </div>
    </div>
  {:else if editorStore.authType === "apiKey"}
    <div class="auth-fields">
      <label class="label" for="apikey-key">Key</label>
      <input
        id="apikey-key"
        type="text"
        class="input"
        placeholder="e.g. X-API-Key"
        bind:value={editorStore.apiKeyKey}
        oninput={() => editorStore.markDirty()}
      />
      <label class="label" for="apikey-value">Value</label>
      <div class="secret-field">
        <input
          id="apikey-value"
          type={showApiKeyValue ? "text" : "password"}
          class="input"
          placeholder="API key value"
          bind:value={editorStore.apiKeyValue}
          oninput={() => editorStore.markDirty()}
        />
        <button
          class="eye-btn"
          title={showApiKeyValue ? "Hide" : "Show"}
          onclick={() => (showApiKeyValue = !showApiKeyValue)}
        >{showApiKeyValue ? "◉" : "○"}</button>
      </div>
      <label class="label" for="apikey-loc">Add to</label>
      <select
        id="apikey-loc"
        class="select"
        value={editorStore.apiKeyLocation}
        onchange={(e) => {
          editorStore.apiKeyLocation = (e.target as HTMLSelectElement).value as ApiKeyLocation;
          editorStore.markDirty();
        }}
      >
        {#each apiKeyLocations as loc}
          <option value={loc.value}>{loc.label}</option>
        {/each}
      </select>
    </div>
  {:else if editorStore.authType === "oauth2"}
    <div class="oauth-section">
      <div class="auth-fields">
        <label class="label" for="oauth-grant">Grant Type</label>
        <select
          id="oauth-grant"
          class="select"
          value={editorStore.oauthGrantType}
          onchange={(e) => {
            editorStore.oauthGrantType = (e.target as HTMLSelectElement).value as OAuthGrantType;
            editorStore.markDirty();
          }}
        >
          {#each grantTypes as g}
            <option value={g.value}>{g.label}</option>
          {/each}
        </select>

        {#if editorStore.oauthGrantType === "authorization_code"}
          <label class="label" for="oauth-auth-url">Auth URL</label>
          <input
            id="oauth-auth-url"
            type="text"
            class="input"
            placeholder="https://provider.com/authorize"
            bind:value={editorStore.oauthAuthUrl}
            oninput={() => editorStore.markDirty()}
          />
        {/if}

        <label class="label" for="oauth-token-url">Token URL</label>
        <input
          id="oauth-token-url"
          type="text"
          class="input"
          placeholder="https://provider.com/token"
          bind:value={editorStore.oauthTokenUrl}
          oninput={() => editorStore.markDirty()}
        />

        <label class="label" for="oauth-client-id">Client ID</label>
        <input
          id="oauth-client-id"
          type="text"
          class="input"
          placeholder="Client ID"
          bind:value={editorStore.oauthClientId}
          oninput={() => editorStore.markDirty()}
        />

        <label class="label" for="oauth-client-secret">Client Secret</label>
        <div class="secret-field">
          <input
            id="oauth-client-secret"
            type={showOAuthSecret ? "text" : "password"}
            class="input"
            placeholder="Client Secret"
            bind:value={editorStore.oauthClientSecret}
            oninput={() => editorStore.markDirty()}
          />
          <button
            class="eye-btn"
            title={showOAuthSecret ? "Hide" : "Show"}
            onclick={() => (showOAuthSecret = !showOAuthSecret)}
          >{showOAuthSecret ? "◉" : "○"}</button>
        </div>

        <label class="label" for="oauth-scopes">Scopes</label>
        <input
          id="oauth-scopes"
          type="text"
          class="input"
          placeholder="read write (space-separated)"
          bind:value={editorStore.oauthScopes}
          oninput={() => editorStore.markDirty()}
        />
      </div>

      <div class="oauth-actions">
        <button
          class="btn btn-primary"
          disabled={oauthLoading || !editorStore.oauthTokenUrl}
          onclick={fetchOAuthToken}
        >
          {oauthLoading ? "Fetching…" : "Get New Token"}
        </button>
        {#if editorStore.oauthRefreshToken}
          <button
            class="btn btn-secondary"
            disabled={oauthLoading}
            onclick={refreshOAuthToken}
          >
            Refresh Token
          </button>
        {/if}
      </div>

      {#if oauthError}
        <p class="oauth-error">{oauthError}</p>
      {/if}

      {#if editorStore.oauthAccessToken}
        <div class="oauth-token-display">
          <span class="label">Access Token</span>
          <code class="token-value">{editorStore.oauthAccessToken.slice(0, 40)}…</code>
          {#if editorStore.oauthTokenExpiry}
            <span class="token-expiry">Expires: {new Date(editorStore.oauthTokenExpiry).toLocaleString()}</span>
          {/if}
        </div>
      {/if}
    </div>
  {:else}
    <p class="no-auth-hint">This request does not use any authentication.</p>
  {/if}
</div>

<style>
  .auth-editor {
    display: flex;
    flex-direction: column;
    gap: var(--sp-md);
  }
  .auth-type-row {
    display: flex;
    align-items: center;
    gap: var(--sp-md);
  }
  .auth-fields {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: var(--sp-sm) var(--sp-md);
    align-items: center;
  }
  .label {
    font-size: var(--fs-small);
    font-weight: 500;
    color: var(--text-secondary);
    white-space: nowrap;
  }
  .input {
    width: 100%;
    font-size: var(--fs-small);
    font-family: var(--font-mono);
  }
  .secret-field {
    display: flex;
    align-items: center;
    gap: var(--sp-xs);
  }
  .secret-field .input {
    flex: 1;
  }
  .eye-btn {
    font-size: var(--fs-body);
    color: var(--text-tertiary);
    padding: var(--sp-xs);
    flex-shrink: 0;
    line-height: 1;
  }
  .eye-btn:hover {
    color: var(--text-primary);
  }
  .select {
    font-size: var(--fs-small);
    padding: var(--sp-xs) var(--sp-sm);
    min-width: 140px;
  }
  .no-auth-hint {
    font-size: var(--fs-small);
    color: var(--text-tertiary);
  }
  .oauth-section {
    display: flex;
    flex-direction: column;
    gap: var(--sp-md);
  }
  .oauth-actions {
    display: flex;
    gap: var(--sp-sm);
  }
  .btn {
    font-size: var(--fs-small);
    padding: var(--sp-xs) var(--sp-md);
    border-radius: var(--radius-sm);
    cursor: pointer;
    white-space: nowrap;
  }
  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  .btn-primary {
    background: var(--accent);
    color: var(--bg-primary);
  }
  .btn-secondary {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }
  .oauth-error {
    font-size: var(--fs-small);
    color: var(--color-error, #ef4444);
  }
  .oauth-token-display {
    display: flex;
    flex-direction: column;
    gap: var(--sp-xs);
    padding: var(--sp-sm);
    background: var(--bg-secondary);
    border-radius: var(--radius-sm);
  }
  .token-value {
    font-size: var(--fs-small);
    font-family: var(--font-mono);
    word-break: break-all;
    color: var(--text-secondary);
  }
  .token-expiry {
    font-size: var(--fs-xsmall, 11px);
    color: var(--text-tertiary);
  }
</style>
