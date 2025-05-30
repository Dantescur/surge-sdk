// tests/client.rs
use serde_json::json;
use surge_sdk::{Auth, SurgeError};
use tempfile::tempdir;
use tokio::fs;

mod common;
use common::TestServer;

#[tokio::test]
async fn test_login_success() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("POST", "/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "email": "test@example.com",
                "token": "abc123"
            })
            .to_string(),
        )
        .create_async()
        .await;

    let response = test_server
        .client
        .login(&Auth::UserPass {
            username: "test@example.com".to_string(),
            password: "password".to_string(),
        })
        .await
        .unwrap();

    assert_eq!(response.email, "test@example.com");
    assert_eq!(response.token, "abc123");
}

#[tokio::test]
async fn test_login_failure() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("POST", "/token")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "errors": ["Invalid credentials"],
                "details": {},
                "status": 401
            })
            .to_string(),
        )
        .create_async()
        .await;

    let result = test_server
        .client
        .login(&Auth::UserPass {
            username: "test@example.com".to_string(),
            password: "wrong".to_string(),
        })
        .await;

    assert!(matches!(result, Err(SurgeError::Http(_))));
}

#[tokio::test]
async fn test_account_success() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("GET", "/account")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "email": "test@example.com",
                "id": "123",
                "uuid": "uuid-123",
                "role": 5,
                "updated_at": "2025-05-29T00:00:00Z",
                "created_at": "2025-05-29T00:00:00Z",
                "payment_id": null,
                "email_verified_at": null,
                "stripe": null,
                "plan": {
                    "id": "student-00",
                    "name": "Student",
                    "amount": "0000",
                    "friendly": "student",
                    "dummy": true,
                    "current": true,
                    "metadata": { "type": "account" },
                    "ext": "00",
                    "perks": ["Unlimited projects"],
                    "comped": false
                },
                "card": null
            })
            .to_string(),
        )
        .create_async()
        .await;

    let response = test_server
        .client
        .account(&Auth::Token("abc123".to_string()))
        .await
        .unwrap();

    assert_eq!(response.email, "test@example.com");
    assert_eq!(response.plan.id, "student-00");
}

#[tokio::test]
async fn test_list_no_domain() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("GET", "/list")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!([{
                "domain": "test.surge.sh",
                "planName": {
                    "name": "Standard",
                    "id": "3",
                    "amount": 22,
                    "friendly": "yes",
                    "dummy": false,
                    "current": true,
                    "metadata": {
                        "type": "info",
                        "extra": "tired"
                }},
                "rev": 123456,
                "cmd": "surge",
                "email": "test@example.com",
                "platform": "surge.sh",
                "cliVersion": "0.1.0",
                "output": {},
                "config": { "settings": {} },
                "message": null,
                "buildTime": null,
                "ip": "127.0.0.1",
                "privateFileList": [],
                "publicFileCount": 5,
                "publicTotalSize": 1000,
                "privateFileCount": 5,
                "privateTotalSize": 1000,
                "uploadStartTime": 1234567890,
                "uploadEndTime": 1234567891,
                "uploadDuration": 1.0,
                "preview": null,
                "timeAgoInWords": "Just now"
            }])
            .to_string(),
        )
        .create_async()
        .await;

    let response = test_server
        .client
        .list(None, &Auth::Token("abc123".to_string()))
        .await
        .unwrap();

    assert_eq!(response.len(), 1);
    assert_eq!(response[0].domain, "test.surge.sh");
}

#[tokio::test]
async fn test_teardown_success() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("DELETE", "/test.surge.sh")
        .with_status(200)
        .create_async()
        .await;

    let result = test_server
        .client
        .teardown("test.surge.sh", &Auth::Token("abc123".to_string()))
        .await
        .unwrap();

    assert!(result);
}

#[tokio::test]
async fn test_publish_metadata() {
    let _test_server = TestServer::new().await;
    let dir = tempdir().unwrap();
    let project_path = dir.path();
    fs::write(project_path.join("file1.txt"), "hello")
        .await
        .unwrap();
    fs::write(project_path.join("file2.txt"), "world")
        .await
        .unwrap();

    let metadata = surge_sdk::calculate_metadata(project_path).unwrap();
    assert_eq!(metadata.file_count, 2);
    assert_eq!(metadata.project_size, 10); // "hello" + "world" = 10 bytes
}

#[tokio::test]
async fn test_dns() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("GET", "/test.surge.sh/dns")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "message": "DNS may only be managed on apex domains."
            })
            .to_string(),
        )
        .create_async()
        .await;

    let response = test_server
        .client
        .dns("test.surge.sh", &Auth::Token("abc123".to_string()))
        .await
        .unwrap();

    assert_eq!(
        response["message"],
        "DNS may only be managed on apex domains."
    );
}
