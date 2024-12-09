use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use actix_files::Files;
use serde::{Serialize};
use rand::Rng;
use std::process::Command;
use std::path::PathBuf;

#[derive(Serialize)]
struct Response {
    number: u32,
    video_path: Option<String>,
    message: String,
}

async fn play_video(media_path: &str) -> bool {
    println!("Attempting to play video: {}", media_path);

    let absolute_path = PathBuf::from(media_path)
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from(media_path));
    
    println!("Absolute path: {:?}", absolute_path);

    let mpv_path = String::from_utf8(
        Command::new("which")
            .arg("mpv")
            .output()
            .expect("Failed to execute which")
            .stdout
    ).expect("Failed to convert path to string")
    .trim()
    .to_string();

    println!("Using MPV path: {}", mpv_path);
    println!("Launching MPV...");

    let result = Command::new(&mpv_path)
        // Basic options
        .arg("--fullscreen")
        .arg("--no-terminal")
        
        // Video output options
        .arg("--vo=gpu-next")
        .arg("--gpu-context=x11egl")
        .arg("--hwdec=vaapi")
        
        // Basic audio options
        .arg("--audio-device=auto")
        .arg("--audio-channels=stereo")
        .arg("--volume-max=200")
        
        // The video file
        .arg(absolute_path.to_str().unwrap())
        .output();

    match result {
        Ok(output) => {
            println!("MPV command executed");
            if !output.status.success() {
                println!("MPV command failed with status: {}", output.status);
                println!("Error output: {}", String::from_utf8_lossy(&output.stderr));
                println!("Standard output: {}", String::from_utf8_lossy(&output.stdout));
            }
            output.status.success()
        },
        Err(e) => {
            println!("Failed to execute MPV command: {}", e);
            false
        }
    }
}

#[get("/generate")]
async fn generate_number() -> Result<HttpResponse> {
    let random_number: u32 = {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=11)
    };

    let (video_path, message) = match random_number {
        11 => (Some("assets/video4.mp4".to_string()), 
              "P diddy loves you".to_string()),
        5..=7 => (Some("assets/video1.mp4".to_string()), 
                format!("Number Of Inches: {}", random_number)),
        1..=4 => (Some("assets/video2.mp4".to_string()), 
                format!("Number Of Inches: {}", random_number)),
        8..=10 => (Some("assets/video3.mp4".to_string()), 
                 format!("Number Of Inches: {}", random_number)),
        _ => (None, "Invalid number".to_string()),
    };

    Ok(HttpResponse::Ok().json(Response {
        number: random_number,
        video_path,
        message,
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(generate_number)
            .service(Files::new("/assets", "assets").show_files_listing())
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

