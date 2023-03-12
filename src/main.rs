use std::env;
use std::path::Path;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder };
use std::process::Command;

async fn run_script() -> impl Responder {
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
  HttpServer::new(|| App::new().route("/", web::get().to(run_script)))
    .bind("127.0.0.1:3000")?
    .run().await
}