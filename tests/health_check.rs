use std::net::TcpListener;
use sqlx::{PgConnection, PgPool, Connection, Executor};
use img_service::configuration::{get_configuration, DatabaseSettings};
use uuid::Uuid;


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}


pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to PG");
    
    connection
        .execute( format! (r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool:: connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind rand port");
    let port = listener.local_addr().unwrap().port();

    let mut cfg = get_configuration().expect("Failed to read cfg");
    cfg.database.database_name = Uuid::new_v4().to_string();
    let conn_pool = configure_database(&cfg.database).await;

    let server = img_service::run(listener, conn_pool.clone()).expect("Failed to bind addr");
    let _ = tokio::spawn(server);

    TestApp{
        address: format!("http://127.0.0.1:{}", port), 
        db_pool: conn_pool
    }
}

#[tokio::test]
async fn check_health_check() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscribe", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to exec request");
    
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}


#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];


    for (invalid_body, err_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        
        assert_eq!(
            400,
            response.status().as_u16(),
            "Expected 400 Bad Request, got err msg {}.",
            err_msg
        )
    }
}

