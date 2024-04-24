use anyhow::{bail, Context, Result};
use marzano_auth::{env::get_graphql_api_url, info::AuthInfo};
use marzano_gritmodule::fetcher::ModuleRepo;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{mem, str::FromStr};

#[derive(Serialize, Debug)]
struct RepoInput<'a> {
    repo: &'a str,
    host: &'a str,
}

#[derive(Serialize)]
struct RulesQuery<'a> {
    query: &'a str,
    variables: RepoInput<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutoReviewRule {
    id: String,
    title: String,
    description: String,
    level: String,
    created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RuleProject {
    review_rules: Vec<AutoReviewRule>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RulesData {
    pattern_analysis_project: Vec<RuleProject>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RulesResponse {
    data: RulesData,
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

#[allow(dead_code)]
async fn fetch_project_rules(repo: &ModuleRepo, auth: &AuthInfo) -> Result<Vec<AutoReviewRule>> {
    let token = &auth.access_token;
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
        .bearer_auth(token)
        .json(&graphql_query)
        .send()
        .await?;

    let response_body = res.text().await?;
    let mut response = serde_json::from_str::<RulesResponse>(&response_body)
        .context("Failed to parse rules response")?;
    let project = response
        .data
        .pattern_analysis_project
        .get_mut(0)
        .context("No project found")?;
    let rules = mem::take(&mut project.review_rules);

    Ok(rules)
}
