use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use exif::{In, Tag};


const EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "JPG", "JPEG", "png", "PNG"];


pub fn run_for(input: &str, output: &str) -> Result<(), Box<dyn Error>> {
    let ext_set: HashSet<&str> = HashSet::from(EXTENSIONS);
    let (str_i, str_o) = prepare_paths(input, output)?;

    for entry in fs::read_dir(&str_i)? {
        let file_path = entry?.path();
        let file_name = file_path.file_name();
        let extension = file_path.extension().and_then(|ext| ext.to_str());
        if !file_path.is_file() || file_name.is_none() || extension.is_none() || !ext_set.contains(extension.unwrap()) {
            match file_name.unwrap().to_str() {
                None => println!("Skipping 1 file..."),
                Some(name) => println!("Skipping {}", name),
            }
            continue
        }

        let out_temp = str_o.join(&file_name.unwrap());
        let exif_name = extract_exif_name(&file_path).unwrap();
        let out_name = str_o.join(exif_name).with_extension(&extension.unwrap());

        fs::copy(&file_path, &out_temp)?;
        fs::rename(&out_temp, &out_name)?;
    }
    Ok(())
}


fn prepare_paths(input: &str, output: &str) -> Result<(PathBuf, PathBuf), Box<dyn Error>> {
    let path_i = Path::new(input).canonicalize()?;
    let path_o = Path::new(output);
    fs::create_dir_all(&path_o)?;
    let path_o = path_o.canonicalize()?;
    Ok((path_i, path_o))
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
