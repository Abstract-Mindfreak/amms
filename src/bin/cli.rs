use mmss::core::semantic_task_processor::SemanticTaskProcessor;
use mmss::core::types::{GeometricOperator, GeometricTaskCommand};

fn main() {
    env_logger::init();
    println!("MMSS CLI placeholder");

    let processor = SemanticTaskProcessor::new();

    let task = GeometricTaskCommand {
        task_name: "Inspect Quaternion Cohesion".to_string(),
        geometric_operator: GeometricOperator::QuaternionRotation,
        target_module: "emergence_logic".to_string(),
        parameters: serde_json::json!({ "sample": "placeholder" }),
        expected_output_metric: "v_geometric".to_string(),
        task_id: None,
    };

    match processor.submit_task(task) {
        Ok(task_id) => {
            println!("Submitted task {task_id}");
            if let Ok(result) = processor.execute_task(task_id) {
                println!("Task success: {}", result.success);
                println!("{:?}", result.metrics);
            }
        }
        Err(err) => eprintln!("Failed to submit task: {err}"),
    }
}
