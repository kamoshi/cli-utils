use std::collections::HashSet;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use exif::{In, Tag};


const EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "JPG", "JPEG", "png", "PNG"];


pub fn run_for(dir: &str) -> Result<(), Box<dyn Error>> {
    let ext_set: HashSet<&str> = HashSet::from(EXTENSIONS);
    let str_dir = Path::new(dir).canonicalize()?;

    for entry in fs::read_dir(&str_dir)? {
        let file_path = entry?.path();
        let file_name = file_path.file_name();
        let extension = file_path.extension().and_then(|ext| ext.to_str());
        if !file_path.is_file() || file_name.is_none() || extension.is_none() || !ext_set.contains(extension.unwrap()) {
            print_error(file_name.unwrap(), "Skipping");
            continue
        }

        let exif_name = match extract_exif_name(&file_path) {
            Ok(exif) => exif,
            Err(err) => {
                print_error(file_name.unwrap(), &err.to_string());
                continue
            }
        };

        let out_path = str_dir.join(exif_name).with_extension(&extension.unwrap());
        if Path::new(out_path.as_path()).exists() {
            print_error(out_path.as_os_str(), "Already exists, skipping");
        }

        fs::rename(&file_path, &out_path)?;
    }
    Ok(())
}


fn print_error(name: &OsStr, info: &str) {
    match name.to_str() {
        None => eprintln!("Unknown file: {}", info),
        Some(name) => eprintln!("{}: {}", name, info),
    }
}


fn extract_exif_name<T>(path: T) -> Result<String, Box<dyn Error>> where T: AsRef<Path> {
    let file = File::open(path)?;
    let mut bufreader = BufReader::new(&file);
    let exif_reader = exif::Reader::new();
    let exif = exif_reader.read_from_container(&mut bufreader)?;

    let date = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY).ok_or("No EXIF date")?;
    let date = date.display_value().to_string();

    let mut output = String::with_capacity(15);
    for char in date.chars() {
        match char {
            '-' => continue,
            ':' => continue,
            ' ' => output.push('-'),
            any => output.push(any),
        }
    }
    Ok(output)
}
