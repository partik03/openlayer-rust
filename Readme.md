# OpenLayer API Client

This Rust library provides an interface to interact with the OpenLayer API for managing inference pipelines, including creating, updating, retrieving, listing, and deleting inference pipelines and their test results.

## Features

- List inference pipelines.
- Retrieve a specific inference pipeline.
- Create, update, and delete inference pipelines.
- Publish individual inference data points.
- Update existing inference data points.
- List the latest test results for an inference pipeline.

## Requirements

- Rust (version 1.56 or higher)
- `tokio` for asynchronous runtime
- `reqwest` for making HTTP requests
- `serde` and `serde_json` for serialization and deserialization

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/openlayer-api-client.git
   cd openlayer-api-client
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Add the necessary dependencies to your `Cargo.toml`:
   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["json"] }
   tokio = { version = "1", features = ["full"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```

## Usage

### Setting Up

1. Replace `your_api_token` in the application with your actual API token.

### Example Functions

1. **List Inference Pipelines**:
   - Retrieve a list of inference pipelines in a specified project.

2. **Retrieve Inference Pipeline**:
   - Get details of a specific inference pipeline by its ID.

3. **Update Inference Pipeline**:
   - Modify the properties of an existing inference pipeline.

4. **Delete Inference Pipeline**:
   - Remove an inference pipeline from your project.

5. **Publish Inference**:
   - Stream an individual inference data point to an inference pipeline.

6. **Update Inference**:
   - Update an existing inference data point in an inference pipeline.

7. **List Pipeline Test Results**:
   - Get the latest test results for a specific inference pipeline.

### Running the Application

Run the application using:
```bash
cargo run
```

The application will execute the configured functions and output the results.

## API Reference

### Authorization

All API calls require Bearer token authentication in the header:
```http
Authorization: Bearer <token>
```

### API Endpoints

#### 1. List Inference Pipelines
- **Endpoint**: `GET https://api.openlayer.com/v1/projects/{projectId}/inference-pipelines`

#### 2. Retrieve Inference Pipeline
- **Endpoint**: `GET https://api.openlayer.com/v1/inference-pipelines/{inferencePipelineId}`

#### 3. Update Inference Pipeline
- **Endpoint**: `PUT https://api.openlayer.com/v1/inference-pipelines/{inferencePipelineId}`

#### 4. Delete Inference Pipeline
- **Endpoint**: `DELETE https://api.openlayer.com/v1/inference-pipelines/{inferencePipelineId}`

#### 5. Publish Inference
- **Endpoint**: `POST https://api.openlayer.com/v1/inference-pipelines/{inferencePipelineId}/data-stream`

#### 6. Update Inference
- **Endpoint**: `PUT https://api.openlayer.com/v1/inference-pipelines/{inferencePipelineId}/rows?inferenceId={inferenceId}`

#### 7. List Pipeline Test Results
- **Endpoint**: `GET https://api.openlayer.com/v1/inference-pipelines/{inferencePipelineId}/results`

### Query Parameters

- `page`: The page to return in a paginated query (default: 1).
- `perPage`: Maximum number of items to return per page (default: 25).
- `type`: Filter objects by test type. Available options: `integrity`, `consistency`, `performance`, `fairness`, `robustness`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
```

### Explanation of Sections

- **Project Title and Description**: Introduces the library and its purpose.
- **Features**: Highlights the capabilities of the library.
- **Requirements**: Lists necessary software and libraries.
- **Installation**: Provides instructions to clone, build, and set up the library.
- **Usage**: Instructions on setting up the API token and an overview of example functions.
- **Running the Application**: Command to execute the application.
- **API Reference**: Detailed description of authorization requirements and endpoints.
- **License**: Licensing information.

Feel free to customize the GitHub URL and any other details specific to your library. This README will help users understand the functionality and how to use the OpenLayer API client effectively.