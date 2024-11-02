use reqwest::{header::AUTHORIZATION, Client, Error, Method};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::types::development_types::*;

pub struct Development {
    client: Client,
    base_url: String,
}

impl Development {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.openlayer.com/v1".to_string(),
        }
    }
    // Define the function to create a project commit
    pub async fn create_project_commit(
        &self,
        token: &str,
        project_id: &str,
        storage_uri: &str,
        commit_message: &str,
        deployment_status: Option<&str>,
        archived: Option<bool>,
    ) -> Result<CreateCommitResponse, Error> {
        let url = format!("{}/projects/{}/versions", self.base_url, project_id);

        let request_body = CreateCommitRequest {
            storage_uri: storage_uri.to_string(),
            commit: CommitDetails {
                message: commit_message.to_string(),
            },
            deployment_status: deployment_status.map(|s| s.to_string()),
            archived,
        };

        let request_body_json = serde_json::to_string(&request_body)
            .map_err(|e| format!("Failed to serialize request body: {}", e))?;

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(request_body_json)
            .send()
            .await?
            .json::<CreateCommitResponse>()
            .await?;

        Ok(response)
    }

    // Define the function to list commits
    pub async fn list_project_commits(
        &self,
        token: &str,
        project_id: &str,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<ListCommitsResponse, Error> {
        let url = format!("{}/projects/{}/versions", self.base_url, project_id);

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .query(&[
                ("page", page.unwrap_or(1)),
                ("perPage", per_page.unwrap_or(25)),
            ])
            .send()
            .await?
            .json::<ListCommitsResponse>()
            .await?;

        Ok(response)
    }

    // Define the function to retrieve the presigned URL
    pub async fn get_presigned_url(
        &self,
        token: &str,
        object_name: &str,
    ) -> Result<PresignedUrlResponse, Error> {
        let url = format!(
            "{}/storage/presigned-url?objectName={}",
            self.base_url, object_name
        );

        // Send the POST request
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .json::<PresignedUrlResponse>()
            .await?;

        Ok(response)
    }

    // Define the function to list commit test results
    pub async fn list_commit_test_results(
        &self,
        token: &str,
        project_version_id: &str,
        page: Option<i32>,
        per_page: Option<i32>,
        test_type: Option<&str>,
        include_archived: Option<bool>,
    ) -> Result<ListTestResultsResponse, Error> {
        let url = format!("{}/versions/{}/results", self.base_url, project_version_id);

        // Build the query parameters
        let mut query_params = reqwest::Url::parse(&url)?.query_pairs_mut();
        if let Some(p) = page {
            query_params.append_pair("page", &p.to_string());
        }
        if let Some(pp) = per_page {
            query_params.append_pair("perPage", &pp.to_string());
        }
        if let Some(tt) = test_type {
            query_params.append_pair("type", tt);
        }
        if let Some(archived) = include_archived {
            query_params.append_pair("includeArchived", &archived.to_string());
        }

        let response = self
            .client
            .get(query_params.finish().as_str())
            .bearer_auth(token)
            .send()
            .await?
            .json::<ListTestResultsResponse>()
            .await?;

        Ok(response)
    }
}