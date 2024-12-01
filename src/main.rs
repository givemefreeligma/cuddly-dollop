use macroquad::prelude::*;
use std::process::Command;
use ::rand::{Rng, thread_rng};

// Need to change main to return Result from macroquad
#[macroquad::main("My Window")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let random_number: u32 = rng.gen_range(1..10);
    
    // Main game loop
    loop {
        clear_background(WHITE);
        
        // Draw text with the random number
        draw_text(
            &format!("Number Of Inches: {}", random_number),
            20.0,
            40.0,
            30.0,
            BLACK
        );
        
        // Your existing logic can go here, but you'll need to trigger it with a condition
        // For example, you might want to play the videos when a key is pressed
        if is_key_pressed(KeyCode::Space) {
            // Your existing video playing logic here
            if random_number >= 5 && random_number <= 7 {
                print!("long penis");
                let vlc_path = "vlc"; // On Windows, you may need to use "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

                // Path to the media file you want to play
                let media_path = "/home/gooner/cuddly-dollop/2024-11-28 17-47-20.mkv"; // Replace with actual video file path

                // Command to start VLC with the media file
                let status = Command::new(vlc_path)
                    .arg(media_path) // Pass the media file to VLC
                    .spawn() // Start the VLC process
                    .expect("Failed to start VLC player")
                    .wait() // Wait for VLC to finish playing
                    .expect("Failed to wait on VLC");

                if status.success() {
                    println!("VLC played the video successfully.");
                } else {
                    println!("VLC did not play the video successfully.");
                }
            } 
			else if random_number < 5 {
				println!("small penis");
                let vlc_path = "vlc"; // On Windows, you may need to use "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

                // Path to the media file you want to play
                let media_path = "/home/gooner/cuddly-dollop/2024-11-30 17-52-14.mkv"; // Replace with actual video file path

                // Command to start VLC with the media file
                let status = Command::new(vlc_path)
                    .arg(media_path) // Pass the media file to VLC
                    .spawn() // Start the VLC process
                    .expect("Failed to start VLC player")
                    .wait() // Wait for VLC to finish playing
                    .expect("Failed to wait on VLC");

                if status.success() {
                    println!("VLC played the video successfully.");
                } else {
                    println!("VLC did not play the video successfully.");
                }

			}



		
				
				
				
				
				
			}
            
			else if random_number > 7 {
				println!("you won the lottery");
                let vlc_path = "vlc"; // On Windows, you may need to use "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

                // Path to the media file you want to play
                let media_path = "/home/gooner/cuddly-dollop/2024-11-30 17-52-14.mkv"; // Replace with actual video file path

                // Command to start VLC with the media file
                let status = Command::new(vlc_path)
                    .arg(media_path) // Pass the media file to VLC
                    .spawn() // Start the VLC process
                    .expect("Failed to start VLC player")
                    .wait() // Wait for VLC to finish playing
                    .expect("Failed to wait on VLC");

                if status.success() {
                    println!("VLC played the video successfully.");
                } else {
                    println!("VLC did not play the video successfully.");
                }

            // ... rest of your conditions ...
        }

        next_frame().await;
    }

	







}


