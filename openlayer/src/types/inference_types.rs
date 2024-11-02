use serde::{Serialize, Deserialize};
// Define the structure for the request body
#[derive(Serialize)]
pub struct CreateInferencePipelineRequest {
    pub name: String,
    pub description: Option<String>,
}

// Define the structure for the response
#[derive(Deserialize, Debug)]
pub struct CreateInferencePipelineResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_last_sample_received: Option<String>,
    pub description: Option<String>,
    pub date_last_evaluated: Option<String>,
    pub date_of_next_evaluation: Option<String>,
    pub passing_goal_count: i32,
    pub failing_goal_count: i32,
    pub total_goal_count: i32,
    pub status: String,
    pub status_message: Option<String>,
    pub links: Links,
}

// Define the structure for the links in the response
#[derive(Deserialize, Debug)]
pub struct Links {
    pub app: String,
}


// Define the structure for the response item
#[derive(Deserialize, Debug)]
pub struct InferencePipeline {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_last_sample_received: Option<String>,
    pub description: Option<String>,
    pub date_last_evaluated: Option<String>,
    pub date_of_next_evaluation: Option<String>,
    pub passing_goal_count: i32,
    pub failing_goal_count: i32,
    pub total_goal_count: i32,
    pub status: String,
    pub status_message: Option<String>,
    pub links: Links,
}


// Define the structure for the complete response
#[derive(Deserialize, Debug)]
pub struct ListInferencePipelinesResponse {
    pub items: Vec<InferencePipeline>,
}

// Define the structure for the request body
#[derive(Serialize, Debug)]
pub struct UpdateInferencePipelineRequest {
    pub name: String,
    pub description: Option<String>,
    pub reference_dataset_uri: Option<String>,
}

// Define the structure for the request body
#[derive(Serialize)]
pub struct InferenceRequest {
    pub rows: Vec<InferenceRow>,
    pub config: InferenceConfig,
}

#[derive(Serialize)]
pub struct InferenceRow {
    pub user_query: String,
    pub output: String,
    pub tokens: i32,
    pub cost: f64,
    pub timestamp: i64,
}

#[derive(Serialize)]
pub struct InferenceConfig {
    pub prompt: Vec<Prompt>,
    pub input_variable_names: Vec<String>,
    pub output_column_name: String,
    pub timestamp_column_name: String,
    pub cost_column_name: String,
    pub num_of_token_column_name: String,
}

#[derive(Serialize)]
pub struct Prompt {
    pub role: String,
    pub content: String,
}


// Define the structure for the request body
#[derive(Serialize)]
pub struct UpdateInferenceRequest {
    pub row: UpdateInferenceRow,
    pub config: UpdateInferenceConfig,
}

#[derive(Serialize)]
pub struct UpdateInferenceRow {
    pub ground_truth: String,
}

#[derive(Serialize)]
pub struct UpdateInferenceConfig {
    pub ground_truth_column_name: Option<String>,
    pub latency_column_name: Option<String>,
    pub timestamp_column_name: Option<String>,
    pub inference_id_column_name: Option<String>,
    pub human_feedback_column_name: Option<String>,
}

// Define the structure for the test result items
#[derive(Deserialize)]
pub struct TestResult {
    pub id: String,
    pub goal: Option<Goal>,
    pub project_version_id: String,
    pub inference_pipeline_id: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_data_starts: Option<String>,
    pub date_data_ends: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
}

#[derive(Deserialize)]
pub struct Goal {
    pub goal_id: Option<String>,
}

// Define the structure for the response
#[derive(Deserialize)]
pub struct ListTestResultsResponse {
    pub items: Vec<TestResult>,
}
