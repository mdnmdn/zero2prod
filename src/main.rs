use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
  let bind_address = "127.0.0.1:8000";
  let listener = std::net::TcpListener::bind(bind_address).expect("Error opening tcp port");
  println!("Starting server on http://{}", listener.local_addr().unwrap());
  run(listener)?.await
}

