async fn spawn_app() -> std::io::Result<()>{
   zero2prodLibrary::run().await
}

#[tokio::test]
async fn health_check_works(){
    let client = reqwest::Client::new();
    let response = client
                                     .get("http://127.0.0.1:8000/healthcheck")
                                     .send()
                                     .await
                                     .expect("unable to send request");
        assert!(response.status().is_success());
        assert_eq!(Some(0) , response.content_length())  
}