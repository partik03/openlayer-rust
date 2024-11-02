use reqwest::{header::AUTHORIZATION, Client, Error};
use crate::types::inference_types::*;

pub struct Inference {
    client: Client,
    base_url: String,
}

impl Inference {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.openlayer.com/v1".to_string(),
        }
    }

    // Define the function to list inference pipelines
    pub async fn list_inference_pipelines(
        &self,
        token: &str,
        project_id: &str,
        page: Option<u32>,
        per_page: Option<u32>,
        name: Option<&str>,
    ) -> Result<ListInferencePipelinesResponse, Error> {
        let mut url = format!(
            "{}/projects/{}/inference-pipelines",
            self.base_url, project_id
        );

        // Append query parameters if they are provided
        let mut query_params = vec![];

        if let Some(p) = page {
            query_params.push(format!("page={}", p));
        }

        if let Some(pp) = per_page {
            query_params.push(format!("perPage={}", pp));
        }

        if let Some(n) = name {
            query_params.push(format!("name={}", n));
        }

        if !query_params.is_empty() {
            url.push_str(&format!("?{}", query_params.join("&")));
        }

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .json::<ListInferencePipelinesResponse>()
            .await?;

        Ok(response)
    }

    // Define the function to retrieve an inference pipeline
    pub async fn retrieve_inference_pipeline(
        &self,
        token: &str,
        inference_pipeline_id: &str,
    ) -> Result<InferencePipeline, Error> {
        let url = format!(
            "{}/inference-pipelines/{}",
            self.base_url, inference_pipeline_id
        );

        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .json::<InferencePipeline>()
            .await?;

        Ok(response)
    }

    // Define the function to create an inference pipeline
    pub async fn create_inference_pipeline(
        &self,
        token: &str,
        project_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<CreateInferencePipelineResponse, Error> {
        let url = format!(
            "{}/projects/{}/inference-pipelines",
            self.base_url, project_id
        );

        // Create the request body
        let request_body = CreateInferencePipelineRequest {
            name: name.to_string(),
            description: description.map(|desc| desc.to_string()),
        };

        let request_body_json = serde_json::to_string(&request_body);

        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .header("Content-Type", "application/json")
            .body(request_body_json)
            .send()
            .await?
            .json::<CreateInferencePipelineResponse>()
            .await?;

        Ok(response)
    }

    // Define the function to update an inference pipeline
    pub async fn update_inference_pipeline(
        &self,
        token: &str,
        inference_pipeline_id: &str,
        name: &str,
        description: Option<&str>,
        reference_dataset_uri: Option<&str>,
    ) -> Result<InferencePipeline, Error> {
        let url = format!(
            "{}/inference-pipelines/{}",
            self.base_url, inference_pipeline_id
        );

        let request_body = UpdateInferencePipelineRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            reference_dataset_uri: reference_dataset_uri.map(|s| s.to_string()),
        };

        let request_body_json = serde_json::to_string(&request_body);

        let response = self
            .client
            .put(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(request_body_json)
            .send()
            .await?
            .json::<InferencePipeline>()
            .await?;

        Ok(response)
    }

    // Define the function to delete an inference pipeline
    pub async fn delete_inference_pipeline(
        &self,
        token: &str,
        inference_pipeline_id: &str,
    ) -> Result<(), Error> {
        let url = format!(
            "{}/inference-pipelines/{}",
            self.base_url, inference_pipeline_id
        );

        // Send a DELETE request
        self.client
            .delete(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .error_for_status()?; // This will return an error if the status is not 2xx

        Ok(())
    }

    // Define the function to publish inference data
    pub async fn publish_inference(
        &self,
        token: &str,
        inference_pipeline_id: &str,
        rows: Vec<InferenceRow>,
        config: InferenceConfig,
    ) -> Result<bool, Error> {
        let url = format!(
            "{}/inference-pipelines/{}/data-stream",
            self.base_url, inference_pipeline_id
        );

        let inference_request = InferenceRequest { rows, config };

        let inference_request_json = serde_json::to_string(&inference_request);

        // Send a POST request
        let response = self
            .client
            .post(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(inference_request_json)
            .send()
            .await?
            .error_for_status()?; // This will return an error if the status is not 2xx

        // Deserialize the response
        let response_body: serde_json::Value = response.json().await?;

        Ok(response_body["success"].as_bool().unwrap_or(false))
    }

    // Define the function to update inference data
    pub async fn update_inference(
        &self,
        token: &str,
        inference_pipeline_id: &str,
        inference_id: &str,
        row: InferenceRow,
        config: InferenceConfig,
    ) -> Result<bool, Error> {
        let url = format!(
            "{}/inference-pipelines/{}/rows?inferenceId={}",
            self.base_url, inference_pipeline_id, inference_id
        );

        let update_request = UpdateInferenceRequest { row, config };

        let update_request_json = serde_json::to_string(&update_request);

        // Send a PUT request
        let response = self
            .client
            .put(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(&update_request_json)
            .send()
            .await?
            .error_for_status()?; // This will return an error if the status is not 2xx

        // Deserialize the response
        let response_body: serde_json::Value = response.json().await?;

        Ok(response_body["success"].as_bool().unwrap_or(false))
    }

    // Define the function to list test results
    pub async fn list_pipeline_test_results(
        &self,
        token: &str,
        inference_pipeline_id: &str,
        page: Option<u32>,
        per_page: Option<u32>,
        test_type: Option<&str>,
    ) -> Result<Vec<TestResult>, Error> {
        let mut url = format!(
            "{}/inference-pipelines/{}/results",
            self.base_url, inference_pipeline_id
        );

        // Append query parameters if provided
        let mut params = vec![];
        if let Some(page) = page {
            params.push(format!("page={}", page));
        }
        if let Some(per_page) = per_page {
            params.push(format!("perPage={}", per_page));
        }
        if let Some(test_type) = test_type {
            params.push(format!("type={}", test_type));
        }

        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }

        // Send a GET request
        let response = self
            .client
            .get(&url)
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .send()
            .await?
            .error_for_status()?; // This will return an error if the status is not 2xx

        // Deserialize the response
        let result: ListTestResultsResponse = response.json().await?;

        Ok(result.items)
    }
}