#[cfg(test)]
mod integration_tests {
    use world_sim_viewer::process_ipc_line;
    use world_sim_viewer::ViewerState;

    #[tokio::test]
    async fn test_ipc_message_parsing() {
        let state = ViewerState::new();

        // Test valid IPC message
        let ipc_message = r#"{"version":1,"timestamp":1234567890,"seq_num":1,"payload":{"Heartbeat":{"sender":"test","sent_at":1234567890,"metrics":null}}}"#;

        // This should not panic
        process_ipc_line(ipc_message, &state).await;

        // Test invalid JSON
        let invalid_json = r#"{"invalid": json}"#;
        process_ipc_line(invalid_json, &state).await;

        // Test non-JSON line
        let non_json = "some random log line";
        process_ipc_line(non_json, &state).await;
    }

    #[tokio::test]
    async fn test_viewer_state_creation() {
        let state = ViewerState::new();
        // Just test that state creation doesn't panic
        assert!(state.clients.read().await.is_empty());
        assert!(state.latest_state.read().await.is_none());
    }
}