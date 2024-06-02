use std::net::TcpListener;

use reqwest::Client;
use sqlx::{Connection, PgConnection};

use zero2prodLibrary::configuration::get_configuration;

fn spawn_app() -> String{
     let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bound to random port ");
     let port = listener.local_addr().unwrap().port();
     let server = zero2prodLibrary::run(listener).expect("Unable to get server");
     let _ = tokio::spawn(server);
     format!("http://127.0.0.1:{}",port)
}

#[tokio::test]
async  fn invalid_subscriptions_returns_400() {
    let address = spawn_app();
    
    let body_contents = vec![
        ("name=bharathi","mail is missing"),
        ("email=bharathi102000%40gmail.com","name is missing"),
        ("","name is missing"),
    ];
    
    for(invalid_body,error_message) in body_contents{
        let client = Client::new();
        let response = client
            .post(format!("{}/subscriptions",&address))
            .header("content-type","application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("unable to send the request");
        assert_eq!(400, response.status().as_u16(),"The Api did not failed with the 400 status when the {}",error_message)
    }
}

#[tokio::test]
async fn health_check_works(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let configraution = get_configuration().expect("Failed to read configuration");
    let connection_string = configraution.database.get_connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Unable to get connection");
    let response = client
                                     .get(format!("{}/healthcheck",&address))
                                     .send()
                                     .await
                                     .expect("unable to send request");
    println!("{}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0) , response.content_length())
}

#[tokio::test]
async fn valid_subscriptions_returns_200()
{
    let app_address =spawn_app();
 
    let configuration = get_configuration().expect(" Unable to get configuration");
    let connection_string = configuration.database.get_connection_string();
    
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    let client = Client::new();

    let body ="name=bharathi&email=bharathi102000%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions",&app_address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //assert
    assert_eq!(200,response.status().as_u16());
    
    let saved = sqlx::query!("SELECT name ,email from subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Cant fetch data from connection");
    
    assert_eq!(saved.email ,"bharathi102000@gmail.com");
    assert_eq!(saved.name ,"bharathi");
    
}

