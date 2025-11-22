use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::semantic_task_processor::TaskStatus;
use crate::core::types::{GeometricTaskCommand, TaskExecutionResult};
use crate::state::AppState;

use super::{bad_request, internal_error, not_found, ApiResult};

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub task: GeometricTaskCommand,
    #[serde(default = "default_execute")]
    pub execute: bool,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub task_id: Uuid,
    pub status: TaskStatus,
    pub execution_result: Option<TaskExecutionResult>,
}

#[derive(Serialize)]
pub struct TaskListItem {
    pub task_id: Uuid,
    pub status: TaskStatus,
}

fn default_execute() -> bool {
    true
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> ApiResult<Json<CreateTaskResponse>> {
    let task_id = state
        .processor
        .submit_task(payload.task)
        .map_err(|err| bad_request(err.to_string()))?;

    if payload.execute {
        let result = state
            .processor
            .execute_task(task_id)
            .map_err(|err| internal_error(err.to_string()))?;

        let response = CreateTaskResponse {
            task_id,
            status: TaskStatus::Completed(result.metrics.clone()),
            execution_result: Some(result),
        };
        Ok(Json(response))
    } else {
        let response = CreateTaskResponse {
            task_id,
            status: TaskStatus::Pending,
            execution_result: None,
        };
        Ok(Json(response))
    }
}

pub async fn list_tasks(State(state): State<AppState>) -> ApiResult<Json<Vec<TaskListItem>>> {
    let tasks = state
        .processor
        .list_tasks()
        .map_err(|err| internal_error(err.to_string()))?;

    let summaries = tasks
        .into_iter()
        .map(|(task_id, status)| TaskListItem { task_id, status })
        .collect();

    Ok(Json(summaries))
}

pub async fn get_task_status(
    Path(task_id): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<TaskListItem>> {
    let id = Uuid::parse_str(&task_id).map_err(|_| bad_request("Invalid task ID"))?;

    let status = state
        .processor
        .get_task_status(id)
        .map_err(|_| not_found("Task not found"))?;

    Ok(Json(TaskListItem {
        task_id: id,
        status,
    }))
}
