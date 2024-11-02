
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
struct InferenceRequest {
    rows: Vec<InferenceRow>,
    config: InferenceConfig,
}

#[derive(Serialize)]
struct InferenceRow {
    user_query: String,
    output: String,
    tokens: i32,
    cost: f64,
    timestamp: i64,
}

#[derive(Serialize)]
struct InferenceConfig {
    prompt: Vec<Prompt>,
    input_variable_names: Vec<String>,
    output_column_name: String,
    timestamp_column_name: String,
    cost_column_name: String,
    num_of_token_column_name: String,
}

#[derive(Serialize)]
struct Prompt {
    role: String,
    content: String,
}


// Define the structure for the request body
#[derive(Serialize)]
struct UpdateInferenceRequest {
    row: UpdateInferenceRow,
    config: UpdateInferenceConfig,
}

#[derive(Serialize)]
struct UpdateInferenceRow {
    ground_truth: String,
}

#[derive(Serialize)]
struct UpdateInferenceConfig {
    ground_truth_column_name: Option<String>,
    latency_column_name: Option<String>,
    timestamp_column_name: Option<String>,
    inference_id_column_name: Option<String>,
    human_feedback_column_name: Option<String>,
}

// Define the structure for the test result items
#[derive(Deserialize)]
struct TestResult {
    id: String,
    goal: Option<Goal>,
    project_version_id: String,
    inference_pipeline_id: String,
    date_created: String,
    date_updated: String,
    date_data_starts: Option<String>,
    date_data_ends: Option<String>,
    status: String,
    status_message: Option<String>,
}

#[derive(Deserialize)]
struct Goal {
    goal_id: Option<String>,
}

// Define the structure for the response
#[derive(Deserialize)]
struct ListTestResultsResponse {
    items: Vec<TestResult>,
}
