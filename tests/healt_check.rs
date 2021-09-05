//! test health check


#[actix_rt::test]
async fn healthcheck_works() {
  let address = spawn_app();

  let client = reqwest::Client::new();
  let url = format!("{}/health_check", address);
  println!("address: {}", url);
  dbg!(&address);
  let response = client
    .get(&format!("{}/health_check", address))
    .send()
    .await
    .expect("Failed to execute request");
  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
  let bind_address = "127.0.0.1";
  let listener = std::net::TcpListener::bind(format!("{}:0", bind_address))
          .expect("Error opening tcp port");
  let port = listener.local_addr().unwrap().port();
  let server = zero2prod::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);
  
  format!("http://{}:{}",bind_address, port)
}
