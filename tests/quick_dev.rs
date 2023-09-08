use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    hc.do_get("/hello2/Correia").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "password": "welcome"
        }),
    );
    req_login.await?.print().await?;

    // Tasks tests
    let req_create_task = hc.do_post(
        "/api/tasks",
        json!({
            "title": "Task 1"
        }),
    );
    req_create_task.await?.print().await?;

    let req_list_tasks = hc.do_get("/api/tasks");
    req_list_tasks.await?.print().await?;

    // let req_delete_task = hc.do_delete("/api/tasks/2");
    // req_delete_task.await?.print().await?;

    Ok(())
}
