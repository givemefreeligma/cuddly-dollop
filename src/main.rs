use std::process::Command;
use rand::Rng;

fn main() {

let mut rng = rand::thread_rng();

let random_number: u32 = rng.gen_range(1..10);

println!("Number Of Inches: {}", random_number);
	











if random_number >= 5 && random_number <= 7 {
	print!("long penis");
	let vlc_path = "vlc"; // On Windows, you may need to use "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

    // Path to the media file you want to play
    let media_path = "2024-11-28 17-47-20.mkv"; // Replace with actual video file path

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
	









	if random_number < 5 {
		println!("what the fuck is that tiny cock lil bro");
		let vlc_path = "vlc"; // On Windows, you may need to use "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

		// Path to the media file you want to play
		let media_path = "2024-11-30 17-52-14.mkv"; // Replace with actual video file path
	
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
	
	












	if random_number > 7 {
		println!("NICE COCK!!!!!!!!!!!!");
		let vlc_path = "vlc"; // On Windows, you may need to use "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe"

		// Path to the media file you want to play
		let media_path = "2024-11-30 17-56-28.mkv"; // Replace with actual video file path
	
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


