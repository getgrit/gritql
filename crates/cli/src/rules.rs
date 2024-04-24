use anyhow::{bail, Context, Result};
use log::info;
use marzano_auth::env::get_graphql_api_url;
use marzano_gritmodule::fetcher::ModuleRepo;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::updater::Updater;

#[derive(Serialize)]
struct RepoInput<'a> {
    repo: &'a str,
    host: &'a str,
}

#[derive(Serialize)]
struct RulesQuery<'a> {
    query: &'a str,
    variables: RepoInput<'a>,
}

pub struct AutoReviewRule {
    id: String,
    title: String,
    description: String,
    level: String,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PatternLevel {
    None = 0,
    Warn = 2,
    Error = 3,
}

impl FromStr for PatternLevel {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "none" => Ok(PatternLevel::None),
            "warn" => Ok(PatternLevel::Warn),
            "error" => Ok(PatternLevel::Error),
            _ => bail!("'{}' is not a valid level", s),
        }
    }
}

async fn fetch_project_rules(repo: &ModuleRepo) -> Result<Vec<AutoReviewRule>> {
    let updater = Updater::from_current_bin().await?;
    let auth = updater.get_auth();
    let token = match auth {
        Some(auth) => {
            if auth.is_expired()? {
                bail!(
                    "Auth token expired: {}. Run grit auth login to refresh.",
                    auth.get_expiry()?
                );
            }
            auth.access_token
        }
        None => {
            bail!("You are not authenticated. Run grit auth login to authenticate.");
        }
    };

    let client = Client::new();
    let query = r#"
        query GetProjectRules($repo: String!, $host: String!) {
            pattern_analysis_project(where: { repo: { _eq: $repo }, host: { _eq: $host }}) {
                id
                review_rules {
                    id
                    title
                    description
                    level
                    created_at
                }
            }
        }
    "#;

    let variables = RepoInput {
        repo: &repo.full_name,
        host: &repo.host,
    };

    let graphql_query = RulesQuery { query, variables };
    let url = format!("{}/graphql", get_graphql_api_url());
    let res = client
        .post(&url)
        .bearer_auth(&token)
        .json(&graphql_query)
        .send()
        .await?;

    let response_body = res.text().await?;
    // deserialize the response
    let response = serde_json::from_str::<serde_json::Value>(&response_body)
        .context("Failed to parse rules response")?;
    println!("Response: {}", response);

    Ok(vec![])
}

#[tokio::test]
async fn test_fetch_project_rules() {
    let repo = ModuleRepo {
        full_name: "custodian-sample-org/demo-shop".to_string(),
        host: "github.com".to_string(),
        remote: "something".to_string(),
        provider_name: "github.com/custodian-sample-org/demo-shop".to_string(),
    };
    match fetch_project_rules(&repo).await {
        Ok(_) => println!("Request sent successfully"),
        Err(e) => println!("Error sending request: {}", e),
    }
    panic!("What the fuck");
}
