use librqbit::{AddTorrent, Session};
use rfd::FileDialog;
use std::io::{self, Write};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt user to select a download directory
    println!("Please select a download directory...");
    let download_dir = FileDialog::new()
        .set_title("Select Download Directory")
        .pick_folder()
        .ok_or("No folder selected")?;
    println!("Selected download directory: {:?}", download_dir);

    // Prompt user to enter a magnet link
    print!("Enter the magnet link: ");
    io::stdout().flush()?; // Ensure the prompt is displayed before reading input

    let mut magnet_link = String::new();
    io::stdin().read_line(&mut magnet_link)?;
    let magnet_link = magnet_link.trim(); // Remove any leading/trailing whitespace
    println!("Received magnet link: {}", magnet_link);

    // Initialize a new session with the selected download path
    println!("Initializing download session...");
    let session = Session::new(download_dir.into()).await?;
    println!("Session initialized successfully.");

    // Add the torrent using the provided magnet link
    println!("Adding torrent to session...");
    let managed_torrent_handle = session
        .add_torrent(AddTorrent::from_url(magnet_link), None)
        .await?
        .into_handle()
        .ok_or("Failed to get torrent handle")?;
    println!("Torrent added successfully. Starting download...");

    // Wait until the torrent download is completed
    managed_torrent_handle.wait_until_completed().await?;
    println!("Download completed!");

    Ok(())
}
