use reqwest::{Client, Error, header::AUTHORIZATION};
use serde::Deserialize;
use std::collections::HashMap;
use crate::types::project_types::*;

pub struct Project {
    client: Client,
    base_url: String,
}

impl Project {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.openlayer.com/v1/projects".to_string(),
        }
    }
    // Define the function to make the request
    pub async fn list_projects(
        &self,
        token: &str,
        page: Option<i32>,
        per_page: Option<i32>,
        name: Option<&str>,
        task_type: Option<&str>,
    ) -> Result<ListProjectsResponse, Error> {
        let mut request = self
            .client
            .get(&self.base_url)
            .header(AUTHORIZATION, format!("Bearer {}", token));

        // Add optional query parameters
        if let Some(page) = page {
            request = request.query(&[("page", page)]);
        }
        if let Some(per_page) = per_page {
            request = request.query(&[("perPage", per_page)]);
        }
        if let Some(name) = name {
            request = request.query(&[("name", name)]);
        }
        if let Some(task_type) = task_type {
            request = request.query(&[("taskType", task_type)]);
        }

        // Send the request and deserialize the JSON response
        let response = request.send().await?.json::<ListProjectsResponse>().await?;
        Ok(response)
    }

    // Define the function to create a new project
    pub async fn create_project(
        &self,
        token: &str,
        name: &str,
        description: Option<&str>,
        task_type: &str,
        git_repo: Option<GitRepo>,
    ) -> Result<Project, Error> {
        // Construct the request payload
        let payload = CreateProjectRequest {
            name: name.to_string(),
            description: description.map(|d| d.to_string()),
            task_type: task_type.to_string(),
            git_repo,
        };

        let json_payload = serde_json::to_string(&payload);

        // Send the POST request
        let response = self
            .client
            .post(&self.base_url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(json_payload)
            .send()
            .await?
            .json::<Project>()
            .await?;

        Ok(response)
    }
}