use std::path::Path;
use std::{fs, io};

fn copy_folder_contents(src: &Path, dest: &Path) -> io::Result<()> {
    if !src.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source folder does not exist",
        ));
    }

    // ensuring that the destination file exists or create it
    if !dest.exists() {
        fs::create_dir(dest)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        // Copy the file or directory
        if entry_path.is_file() {
            fs::copy(&entry_path, &dest_path)?;
            println!("Copied {} to {}", entry_path.display(), dest_path.display());
        } else if entry_path.is_dir() {
            copy_folder_contents(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}
fn main() {
    let source_folder = Path::new("/home/crestfallen/Documents/Second Brain/");
    let destination_folder = Path::new("/home/crestfallen/gdrive/Obsidian/second brain/");

    if let Err(err) = copy_folder_contents(source_folder, destination_folder) {
        println!("Error {err}");
    } else {
        println!("Success");
    }
}
