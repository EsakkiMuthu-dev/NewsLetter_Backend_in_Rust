use std::net::TcpListener;

fn spawn_app() -> String{
     let listener = TcpListener::bind("127.0.0.1:0").expect("Unable to bound to random port ");
     let port = listener.local_addr().unwrap().port();
     let server = zero2prodLibrary::run(listener).expect("Unable to get server");
     let _ = tokio::spawn(server);
     format!("http://127.0.0.1:{}",port)
}

#[tokio::test]
async fn health_check_works(){
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
                                     .get(format!("{}/healthcheck",&address))
                                     .send()
                                     .await
                                     .expect("unable to send request");
    println!("{}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0) , response.content_length())
}