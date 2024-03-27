use anyhow::Result;
use http::HeaderMap;
use std::env;
#[cfg(feature = "network_requests")]
use tokio::runtime::Handle;

/**
 * The execution context is a collection of resources that are injected into the Grit "runtime" when analyzing.
 *
 * This is distinct from the CompilationContext in the compiler,
 * which is meant to be contained within the compiler for internal use.
 *
 * Nothing in the compiler should depend on ExecutionContext.
 * In theory, it should be possible to take a compiled pattern and run it with different execution contexts.
 */
#[cfg(feature = "network_requests")]
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    llm_api: Option<LanguageModelAPI>,
    pub handle: Option<tokio::runtime::Handle>,
    reqwest: reqwest::Client,
    /// Ignore limit patterns - this is important for scans
    pub ignore_limit_pattern: bool,
}

#[cfg(all(
    feature = "network_requests_external",
    feature = "external_functions_ffi",
    not(feature = "network_requests"),
    target_arch = "wasm32"
))]
type FetchFn = fn(url: &str, headers: &HeaderMap, json: &serde_json::Value) -> Result<String>;

#[cfg(all(
    feature = "network_requests_external",
    feature = "external_functions_ffi",
    not(feature = "network_requests"),
    target_arch = "wasm32"
))]
type ExecExternalFn =
    fn(code: &[u8], param_names: Vec<String>, input_bindings: &[&str]) -> Result<Vec<u8>>;

/// This variant of execution context depends on an *external* system making HTTP requests.
/// It is particularly useful for the WebAssembly variant of Marzano.
#[cfg(all(
    feature = "network_requests_external",
    feature = "external_functions_ffi",
    not(feature = "network_requests"),
    target_arch = "wasm32"
))]
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    llm_api: Option<LanguageModelAPI>,
    fetch: FetchFn,
    pub exec_external: ExecExternalFn,
    pub ignore_limit_pattern: bool,
}

#[cfg(not(feature = "network_requests_common"))]
#[derive(Clone, Debug)]
pub struct ExecutionContext {
    llm_api: Option<LanguageModelAPI>,
    pub ignore_limit_pattern: bool,
}

impl ExecutionContext {
    // Fetch the contextual LLM API, or fall back to env vars
    pub fn get_llm_api_or_default(&self) -> Result<LanguageModelAPI> {
        match &self.llm_api {
            Some(api) => Ok(api.clone()),
            None => {
                // Try to build it from the legacy env vars
                let openai_api_key = match env::var("GRIT_OPENAI_API_KEY") {
                    Ok(val) => val,
                    Err(_) => {
                        return Err(anyhow::anyhow!(
                            "Authentication is required. Please run grit auth login or provide the GRIT_OPENAI_API_KEY environment variable."
                        ))
                    }
                };

                Ok(LanguageModelAPI {
                    base_endpoint: "https://api.openai.com/".to_string(),
                    bearer_token: openai_api_key,
                    can_cache: true,
                })
            }
        }
    }
}

impl ExecutionContext {
    #[cfg(feature = "network_requests")]
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(all(
        feature = "network_requests_external",
        feature = "external_functions_ffi",
        not(feature = "network_requests"),
        target_arch = "wasm32"
    ))]
    pub fn new(fetch: FetchFn, exec_external: ExecExternalFn) -> ExecutionContext {
        Self {
            llm_api: None,
            fetch,
            exec_external,
            ignore_limit_pattern: false,
        }
    }

    pub fn with_llm_api(mut self, llm_api: LanguageModelAPI) -> Self {
        self.llm_api = Some(llm_api);
        self
    }

    #[cfg(feature = "network_requests")]
    pub fn send_request(
        &self,
        headers: HeaderMap,
        json: serde_json::Value,
        url: &str,
        token: &str,
    ) -> Result<String> {
        let handle = self.handle.as_ref().ok_or_else(|| {
            anyhow::anyhow!("llm request must be made from within a tokio runtime")
        })?;
        let client = self.reqwest.clone();
        let url = url.to_owned();
        let token = token.to_owned();
        Ok(futures::executor::block_on(async {
            handle
                .spawn(async move {
                    client
                        .post(url)
                        .headers(headers)
                        .bearer_auth(token)
                        .json(&json)
                        .send()
                        .await?
                        .text()
                        .await
                })
                .await
                .expect("Task spawned in Tokio executor panicked")
        })?)
    }

    #[cfg(all(
        feature = "network_requests_external",
        not(feature = "network_requests")
    ))]
    pub fn send_request(
        &self,
        mut headers: HeaderMap,
        json: serde_json::Value,
        url: &str,
        token: &str,
    ) -> Result<String> {
        let fetcher = self.fetch;

        if !token.is_empty() {
            headers.insert("Authorization", format!("Bearer {}", token).parse()?);
        }

        let cache_response = fetcher(url, &headers, &json)?;

        Ok(cache_response)
    }

    #[cfg(not(feature = "network_requests_common"))]
    pub fn send_request(
        &self,
        _headers: HeaderMap,
        _json: serde_json::Value,
        _url: &str,
        _token: &str,
    ) -> Result<String> {
        Err(anyhow::anyhow!(
            "Network requests are disabled in the studio"
        ))
    }
}

impl Default for ExecutionContext {
    #[cfg(feature = "network_requests")]
    fn default() -> Self {
        Self {
            llm_api: None,
            handle: Handle::try_current().ok(),
            reqwest: reqwest::Client::new(),
            ignore_limit_pattern: false,
        }
    }

    #[cfg(all(
        feature = "network_requests_external",
        not(feature = "network_requests")
    ))]
    fn default() -> Self {
        Self {
            llm_api: None,
            fetch: |_url: &str, _headers: &HeaderMap, _json: &serde_json::Value| {
                Err(anyhow::anyhow!("Network requests are disabled"))
            },
            exec_external: |_code: &[u8], _param_names: Vec<String>, _input_bindings: &[&str]| {
                Err(anyhow::anyhow!("External functions are disabled"))
            },
            ignore_limit_pattern: false,
        }
    }

    #[cfg(not(feature = "network_requests_common"))]
    fn default() -> Self {
        Self {
            llm_api: None,
            ignore_limit_pattern: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LanguageModelAPI {
    pub base_endpoint: String,
    pub bearer_token: String,
    pub can_cache: bool,
}
