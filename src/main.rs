use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

fn main() -> io::Result<()> {
    let mut directory = String::new();
    println!("Enter the directory path to organize files\nDefault: Downloads Folder");
    io::stdin().read_line(&mut directory)?;

    if directory.trim().is_empty() {
        if let Some(home_path) = std::env::home_dir() {
            directory = home_path.join("Downloads").to_string_lossy().to_string();
        } else {
            println!("Failed to determine the default Downloads directory.");
            return Ok(());
        }
    }

    let directory = directory.trim();

    if !Path::new(directory).exists() {
        println!("Directory does not exist.");
        return Ok(());
    }

    println!("Organizing files. Press Ctrl+C to stop.");
    organize_files(directory)?;

    Ok(())
}

fn organize_files(directory: &str) -> io::Result<()> {
    let mut folders: HashMap<&str, &str> = HashMap::new();
    folders.insert(".txt", "TextFiles");
    folders.insert(".mp4", "VideoFiles");
    folders.insert(".mkv", "VideoFiles");
    folders.insert(".mov", "VideoFiles");
    folders.insert(".mp3", "AudioFiles");
    folders.insert(".wav", "AudioFiles");
    folders.insert(".m4a", "AudioFiles");
    folders.insert(".py", "PythonFiles");
    folders.insert(".cpp", "CodeFiles");
    folders.insert(".c", "CodeFiles");
    folders.insert(".h", "CodeFiles");
    folders.insert(".li", "CodeFiles");
    folders.insert(".js", "CodeFiles");
    folders.insert(".jpg", "ImageFiles");
    folders.insert(".png", "ImageFiles");
    folders.insert(".webp", "VideoButWeird");
    folders.insert(".avif", "ImageButWeird");
    folders.insert(".jfif", "ImageFiles");
    folders.insert(".jpeg", "ImageFiles");
    folders.insert(".gif", "Gifs");
    folders.insert(".exe", "Executables");
    folders.insert(".zip", "Zips");
    folders.insert(".rar", "Zips");
    folders.insert(".gz", "Zips");
    folders.insert(".7z", "Zips");
    folders.insert(".tar", "Folders");
    folders.insert(".msi", "Installers");
    folders.insert(".jar", "Jars");
    folders.insert(".db", "Databases");
    folders.insert(".ico", "Icons");
    folders.insert(".dll", "Dlls");
    folders.insert(".code-profile", "Profiles");
    folders.insert(".json", "Profiles");
    folders.insert(".odt", "Documents");
    folders.insert(".doc", "Documents");
    folders.insert(".docx", "Documents");
    folders.insert(".xlsx", "Documents");
    folders.insert(".pdf", "Documents");
    folders.insert(".pptx", "Documents");
    folders.insert(".html", "HTML");
    folders.insert(".htm", "HTML");
    folders.insert(".css", "HTML");
    folders.insert(".3mf", "3DPrinting");
    folders.insert(".ct", "CheatEngineTables");
    folders.insert(".CT", "CheatEngineTables");
    folders.insert(".tar.xz", "Folders");

    let blacklisted_files_to_move = vec!["desktop.ini"];

    let mut extensions_with_files = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name() {
            let filename = filename.to_string_lossy();
            if !blacklisted_files_to_move.contains(&filename.as_ref()) {
                if let Some(extension) = path.extension() {
                    let ext_str = format!(".{}", extension.to_string_lossy());
                    if folders.contains_key(ext_str.as_str()) {
                        extensions_with_files.push(ext_str);
                    }
                }
            }
        }
    }

    for ext in &extensions_with_files {
        if let Some(folder_name) = folders.get(ext.as_str()) {
            let folder_path = Path::new(directory).join(folder_name);
            if !folder_path.exists() {
                fs::create_dir(&folder_path)?;
            }
        }
    }

    loop {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(filename) = path.file_name() {
                let filename = filename.to_string_lossy();
                if blacklisted_files_to_move.contains(&filename.as_ref()) {
                    continue;
                }

                if filename != "organize_files.exe" {
                    if let Some(extension) = path.extension() {
                        let ext_str = format!(".{}", extension.to_string_lossy());

                        if ext_str == ".crdownload" {
                            println!("Should {} get deleted? (y/n)", filename);
                            let mut input = String::new();
                            io::stdin().read_line(&mut input)?;
                            if input.trim().to_lowercase() == "y" {
                                fs::remove_file(&path)?;
                                println!("Deleted {}.", filename);
                            } else {
                                println!("Ignored {}.", filename);
                            }
                        } else if ext_str == ".ini" {
                            println!("Ignored {} as it has the '.ini' extension.", filename);
                        } else if let Some(folder_name) = folders.get(ext_str.as_str()) {
                            let dest_folder = Path::new(directory).join(folder_name);
                            let dest_path = dest_folder.join(filename.as_ref());

                            // If file exists, generate new name
                            let final_dest = if dest_path.exists() {
                                let mut rng = rand::thread_rng();
                                let new_name = format!("{}_{}.{}", filename, rng.gen_range(1000..9999), ext_str);
                                dest_folder.join(new_name)
                            } else {
                                dest_path
                            };

                            fs::rename(&path, &final_dest)?;
                            println!("Moved {} to {} folder.", filename, folder_name);
                        } else {
                            println!("Ignored {} as it does not have a recognized extension.", filename);
                        }
                    }
                }
            }
        }

        sleep(Duration::from_secs(10));
    }
}
