use serde::{Deserialize, Serialize};




// Define the response structure
#[derive(Deserialize, Serialize, Debug)]
pub struct PresignedUrlResponse {
    pub url: String,
    pub fields: Option<serde_json::Value>, // Use serde_json::Value to handle dynamic fields
    pub storage_uri: String,
}

// Define the structure of each commit item
#[derive(Deserialize, Serialize, Debug)]
pub struct Commit {
    pub id: String,
    pub date_created: String,
    pub status: String,
    pub status_message: Option<String>,
    pub project_id: String,
    pub archived: Option<bool>,
    pub date_archived: Option<String>,
    pub passing_goal_count: i32,
    pub failing_goal_count: i32,
    pub total_goal_count: i32,
    pub links: CommitLinks,
    pub commit_details: CommitDetails,
    pub ml_model_id: Option<String>,
    pub validation_dataset_id: Option<String>,
    pub training_dataset_id: Option<String>,
}

// Define nested structures for commit details and links
#[derive(Deserialize, Serialize, Debug)]
pub struct CommitDetails {
    pub id: String,
    pub author_id: String,
    pub date_created: String,
    pub file_size: Option<i64>,
    pub message: String,
    pub ml_model_id: Option<String>,
    pub validation_dataset_id: Option<String>,
    pub training_dataset_id: Option<String>,
    pub storage_uri: String,
    pub git_commit_sha: Option<i64>,
    pub git_commit_ref: Option<String>,
    pub git_commit_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommitLinks {
    pub app: String,
}

// Define the response structure for the list of commits
#[derive(Deserialize, Serialize, Debug)]
pub struct ListCommitsResponse {
    pub items: Vec<Commit>,
}

// Define the structure of the request body
#[derive(Deserialize, Debug)]
pub struct CreateCommitRequest {
    pub storage_uri: String,
    pub commit: CommitDetails,
    pub deployment_status: Option<String>,
    pub archived: Option<bool>,
}

// Define the response structure
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCommitResponse {
    pub id: String,
    pub date_created: String,
    pub status: String,
    pub status_message: Option<String>,
    pub project_id: String,
    pub commit: CommitDetails,
    pub deployment_status: Option<String>,
    pub ml_model_id: Option<String>,
    pub validation_dataset_id: Option<String>,
    pub training_dataset_id: Option<String>,
    pub archived: Option<bool>,
    pub date_archived: Option<String>,
    pub passing_goal_count: i32,
    pub failing_goal_count: i32,
    pub total_goal_count: i32,
    pub links: CommitLinks,
}

// Define the structure for the test results response
#[derive(Deserialize, Serialize, Debug)]
pub struct TestResult {
    pub id: String,
    pub goal: Option<GoalDetails>,
    pub inference_pipeline_id: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_data_starts: Option<String>,
    pub date_data_ends: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GoalDetails {
    pub goal_id: Option<String>,
    pub project_version_id: Option<String>,
}

// Define the structure for the overall response
#[derive(Deserialize, Serialize, Debug)]
pub struct ListTestResultsResponse {
    pub items: Vec<TestResult>,
}
