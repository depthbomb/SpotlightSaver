use guid::guid;
use image::ImageFormat;
use std::fs::copy;
use std::fs::{create_dir, File};
use std::io::BufReader;
use std::process::exit;
use winfolder::{known_path, Folder};

#[macro_use]
extern crate guid;

fn main() {
    let mut exit_code = 0;
    #[cfg(not(target_os = "windows"))]
    {
        eprintln!("Spotlight Saver is not supported on this system");
        exit_code = 1;
    }

    #[cfg(target_os = "windows")]
    {
        exit_code = real_main();
    }

    exit(exit_code);
}

#[cfg(target_os = "windows")]
fn real_main() -> i32 {
    let pictures_folder = known_path(&guid! {"33E28130-4E1E-4676-835A-98395C3BC3BB"}).unwrap();
    let destination_folder = pictures_folder.join("Windows Spotlight");
    let mut spotlight_folder = Folder::LocalAppData.path();
    spotlight_folder.push("Packages");
    spotlight_folder.push("Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy");
    spotlight_folder.push("LocalState");
    spotlight_folder.push("Assets");

    if !spotlight_folder.exists() {
        eprintln!("Windows Spotlight assets folder not found. Are you on the appropriate version of Windows?");
        return 1;
    }

    if !destination_folder.exists() {
        if let Err(err) = create_dir(&destination_folder) {
            eprintln!("Unable to create destination folder: {:?}", err);
            return 1;
        }
    }

    for entry in spotlight_folder.read_dir().unwrap() {
        let file_path = entry.unwrap().path();
        let file_buf = File::open(&file_path).unwrap();
        if let Ok(img) = image::load(BufReader::new(file_buf), ImageFormat::Jpeg) {
            if img.width() >= 1920 && img.height() >= 1080 {
                let file_name = file_path.file_stem().unwrap().to_string_lossy();
                let dest_file_path = destination_folder.join(format!("{}.jpg", file_name));
                if dest_file_path.exists() {
                    println!("{:?} already exists, skipping", dest_file_path);
                    continue;
                }

                if let Err(err) = copy(&file_path, &dest_file_path) {
                    eprintln!("Error copying file: {:?}", err);
                } else {
                    println!("Copied {} to {:?}", file_name, dest_file_path);
                }
            }
        }
    }

    return 0;
}
