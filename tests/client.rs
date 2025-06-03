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

    assert!(matches!(
        result,
        Err(SurgeError::Api { .. }) | Err(SurgeError::Http(_))
    ));
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
                "planName": "Plus",
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
                "plansuploadDuratiod": 5,
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

    let list_response = match response {
        surge_sdk::ListResult::Global(list_responses) => list_responses,
        surge_sdk::ListResult::Domain(_) => panic!("No"),
    };

    assert_eq!(list_response.len(), 1);
    assert_eq!(list_response[0].domain, "test.surge.sh");
}

#[tokio::test]
async fn test_teardown_success() {
    let mut test_server = TestServer::new().await;
    let _m = test_server
        .server
        .mock("DELETE", "/test.surge.sh")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            json!({
                "msg": "project removed",
                "nsDomain": "surge.world",
                "instances": [
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "sfo.surgel.sh",
                        "location": "US, San Francisco",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "138.197.235.123",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "lhr.surgel.sh",
                        "location": "GB, London",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "46.101.67.123",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "yyz.surgel.sh",
                        "location": "CA, Toronto",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "159.203.50.177",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "jfk.surgel.sh",
                        "location": "US, New York",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "159.203.159.100",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "ams.surgel.sh",
                        "location": "NL, Amsterdam",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "188.166.132.94",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "fra.surgel.sh",
                        "location": "DE, Frankfurt",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "138.68.112.220",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "sgp.surgel.sh",
                        "location": "SG, Singapore",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "139.59.195.30",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "D.Ocean",
                        "domain": "blr.surgel.sh",
                        "location": "IN, Bangalore",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "139.59.50.135",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "Vultr",
                        "domain": "syd.surgel.sh",
                        "location": "AU, Sydney",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "45.76.126.95",
                        "info": "available"
                    },
                    {
                        "type": "HTTP",
                        "provider": "Linode",
                        "domain": "nrt.surgel.sh",
                        "location": "JP, Tokyo",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "172.104.96.133",
                        "info": "available"
                    },
                    {
                        "type": "NS",
                        "provider": "D.Ocean",
                        "domain": "ns1.surge.world",
                        "location": "US, San Francisco",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "N/A",
                        "info": "available"
                    },
                    {
                        "type": "NS",
                        "provider": "D.Ocean",
                        "domain": "ns2.surge.world",
                        "location": "GB, London",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "N/A",
                        "info": "available"
                    },
                    {
                        "type": "NS",
                        "provider": "D.Ocean",
                        "domain": "ns3.surge.world",
                        "location": "CA, Toronto",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "N/A",
                        "info": "available"
                    },
                    {
                        "type": "NS",
                        "provider": "D.Ocean",
                        "domain": "ns4.surge.world",
                        "location": "US, New York",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green",
                        "ip": "N/A",
                        "info": "available"
                    },
                    {
                        "type": "CNAME",
                        "provider": null,
                        "domain": "geo.surge.world",
                        "location": "⬤ , Any/All",
                        "ip": "N/A",
                        "info": "available",
                        "status": "◍",
                        "statusColor": "green",
                        "confirmation": "✔",
                        "confirmationColor": "green"
                    }
                ]
            })
            .to_string(),
        )
        .create_async()
        .await;

    let result = test_server
        .client
        .teardown("test.surge.sh", &Auth::Token("abc123".to_string()))
        .await
        .unwrap();

    assert_eq!(result.msg, "project removed");
    assert_eq!(result.ns_domain, "surge.world");
    assert_eq!(result.instances.len(), 15);
    assert_eq!(result.instances[0].domain, "sfo.surgel.sh");
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
