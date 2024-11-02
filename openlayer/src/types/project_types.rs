use serde::{Deserialize, Serialize};

// Define the struct for the response from the API
#[derive(Debug, Deserialize)]
pub struct ListProjectsResponse {
    pub items: Vec<Project>,
}

// Define the struct for the request payload
#[derive(Serialize, Debug)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub task_type: String,  // Enum for task types could be used if needed
    pub git_repo: Option<GitRepo>,
}

// Define the struct for the GitRepo object within the request
#[derive(Serialize, Debug)]
pub struct GitRepo {
    pub git_id: i32,
    pub branch: Option<String>,
    pub root_dir: Option<String>,
    pub git_account_id: String,
}

// Define the struct for the response
#[derive(Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub workspace_id: Option<String>,
    pub creator_id: Option<String>,
    pub name: String,
    pub date_created: String,
    pub date_updated: String,
    pub description: Option<String>,
    pub source: Option<String>,
    pub task_type: String,
    pub version_count: i32,
    pub inference_pipeline_count: i32,
    pub goal_count: i32,
    pub development_goal_count: i32,
    pub monitoring_goal_count: i32,
    pub links: Links,
    pub git_repo: Option<GitRepoDetails>,
}

// Define the struct for the Links object within the response
#[derive(Deserialize, Debug)]
pub struct Links {
    pub app: String,
}

// Define the struct for the GitRepo object within the response
#[derive(Deserialize, Debug)]
pub struct GitRepoDetails {
    pub id: String,
    pub git_id: i32,
    pub date_connected: String,
    pub date_updated: String,
    pub branch: Option<String>,
    pub name: String,
    pub private: bool,
    pub slug: String,
    pub url: String,
    pub root_dir: Option<String>,
    pub project_id: String,
    pub git_account_id: String,
}
