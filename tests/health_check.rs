 fn spawn_app() {
   let server = zero2prodLibrary::run().expect("Unable to get server");
    tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works(){
    spawn_app();
    let client = reqwest::Client::new();
    let response = client
                                     .get("http://127.0.0.1:8000/healthcheck")
                                     .send()
                                     .await
                                     .expect("unable to send request");
    println!("{}", response.status());
    assert!(response.status().is_success());
    assert_eq!(Some(0) , response.content_length())  
}