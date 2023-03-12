use std::env;
use std::path::Path;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder, post, get };
use std::process::Command;
use serde::Serialize;

#[derive(Serialize)]
struct MyResponse {
  message: String,
}

#[get("/")]
async fn index() -> impl Responder {
  let response = MyResponse {
    message: "Hello, World!".to_string(),
  };
  HttpResponse::Ok().json(response)
}

#[post("/trigger123")]
async fn webhook(req_body: String) -> impl Responder {
  // Get the current user's home directory
  let home_dir = match dirs::home_dir() {
    Some(home_dir) => home_dir,
    None => panic!("Failed to get home directory"),
  };

  // Set the current working directory
  let new_dir = Path::new(&home_dir);
  env::set_current_dir(&new_dir).expect("Failed to set current dir");

  println!("Current working directory: {:?}", env::current_dir().unwrap());

  let current_dir = env::current_dir().unwrap();
  let script_path = current_dir.join("get-build-deploy.sh");

  let output = Command::new("sh")
    .arg(script_path)
    .output()
    .expect("Failed to execute command");

  let output_str = String::from_utf8_lossy(&output.stdout).to_string();

  HttpResponse::Ok().body(output_str)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Should listen on 0.0.0.0:8000");

  HttpServer::new(|| App::new().service(webhook).service(index))
    .bind("0.0.0.0:8000")?
    .run().await
}