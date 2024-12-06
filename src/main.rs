use macroquad::prelude::*;
use std::process::Command;
use ::rand::Rng;
use std::env;

fn play_video(media_path: &str) -> bool {
    let vlc_path = "vlc";
    
    let status = Command::new(vlc_path)
        .arg(media_path)
        .spawn()
        .expect("Failed to start VLC player")
        .wait()
        .expect("Failed to wait on VLC");

    status.success()
}

fn window_conf() -> Conf {
    Conf {
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let random_number: u32 = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or_else(|| {
            let mut rng = ::rand::thread_rng();
            rng.gen_range(1..12)
        });
    
    let mut timer = 0.0;
    let mut show_second_message = false;

    loop {
        clear_background(WHITE);

        if !show_second_message {
            let text = format!("Number Of Inches: {}", 
                if random_number == 11 { "11:".to_string() } 
                else { random_number.to_string() }
            );

            let font_size = if random_number == 11 { 70.0 } else { 98.0 };
            let text_dimensions = measure_text(&text, None, font_size as u16, 1.0);
            let screen_width = screen_width();
            let screen_height = screen_height();

            draw_text(
                &text,
                (screen_width - text_dimensions.width) / 2.0,
                (screen_height + text_dimensions.height) / 2.0,
                font_size,
                BLACK
            );

            timer += get_frame_time();
            if timer >= 2.0 {
                if random_number == 11 {
                    show_second_message = true;
                    timer = 0.0;  // Reset timer for second message
                } else if random_number >= 5 && random_number <= 7 {
                
                    play_video("assets/video1.mkv");
                    std::process::exit(0);
                } else if random_number < 5 {
                    
                    play_video("assets/video2.mkv");
                    std::process::exit(0);
                } else if random_number > 7 && random_number < 11 {
                
                    play_video("assets/video3.mkv");
                    std::process::exit(0);
                }
            }
        } else {
            // Show Nick Fuentes message
            let text = "P diddy loves you";
            let text_dimensions = measure_text(&text, None, 98, 1.0);
            let screen_width = screen_width();
            let screen_height = screen_height();

            draw_text(
                &text,
                (screen_width - text_dimensions.width) / 2.0,
                (screen_height + text_dimensions.height) / 2.0,
                98.0,
                BLACK
            );

            timer += get_frame_time();
            if timer >= 2.0 {
                play_video("assets/video4.mkv");
                std::process::exit(0);
            }
        }

        next_frame().await;
    }
}

