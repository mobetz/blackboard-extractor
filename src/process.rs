
use std::{fs::{self, read_dir, File}, hash::{DefaultHasher, Hash, Hasher}, io::BufReader, path::PathBuf, str::FromStr};

use anyhow::Error;
use regex::Regex;



pub fn iterate_submissions(unzipped_loc: &PathBuf, anonymize: bool) -> Result<(), anyhow::Error> {

    let assign_pattern = Regex::new(r"(?<assignment>.+)_(?<user>\S_\S+)_attempt_(?<timestamp>\d{4}-\d{2}-\d{2}-\d{2}-\d{2}-\d{2})(?<filename>_.+)?")?;

    for entry in read_dir(unzipped_loc)? {
        let entry = entry?;
        let entry_name = entry.file_name();
        let Some(matches) = assign_pattern.captures(entry_name.to_str().unwrap()) 
           else { return Err(Error::msg(format!("File did not match expected pattern: {entry_name:?}"))); };

           let user_folder = if anonymize { 
                let mut hasher = DefaultHasher::new();
                matches["user"].hash(&mut hasher);
                hasher.finish().to_string()
             } else { 
                matches["user"].to_string() 
            };

           let assign_loc = unzipped_loc.join(&matches["assignment"]).join(user_folder).join(&matches["timestamp"]);
           fs::DirBuilder::new()
                .recursive(true)
                .create(&assign_loc)?;

            

            let new_filename = match matches.name("filename") {
                Some(n) => &n.as_str()[1..], //<- remove preceding _ inside the capture group
                None => "SUBMISSION_DETAILS.txt"
            };

            let file_dest = assign_loc.join(new_filename);
            
            fs::rename(entry.path(), &file_dest)?;

            if file_dest.extension().is_some_and( |ext| ext.eq("zip")) {
                let mut archive = zip::ZipArchive::new(BufReader::new(File::open(&file_dest)?))?;
                archive.extract(&assign_loc)?;
                fs::remove_file(&file_dest)?;
            }
            else if file_dest.extension().is_some_and(|ext| ext.eq("rar")) {
                let archive = unrar::Archive::new(&file_dest);
                let mut opened = archive.open_for_processing()?;
                while let Some(header) = opened.read_header()? {
                    if header.entry().is_file() {
                        opened = header.extract_with_base(&assign_loc)?;
                    } else {
                        opened = header.skip()?;
                    }
                }

                fs::remove_file(&file_dest)?;
            }

    };


    Ok(())
}


pub fn process_blackboard_zip(source: &str, dest: &str, anonymize: bool) -> Result<(), anyhow::Error>{

    let source_path = PathBuf::from_str(source).unwrap();
    let dest_path = PathBuf::from_str(dest).unwrap();

    if !dest_path.exists() {
        fs::DirBuilder::new().create(&dest_path)?;
    }

    if !source_path.exists() || !source_path.extension().is_some_and(|ext| ext.eq("zip")) {
        return Err(Error::msg("Source is not a .zip"));
    }

    let mut archive = zip::ZipArchive::new(BufReader::new(File::open(source_path)?))?;
    
    archive.extract(&dest_path)?;


    iterate_submissions(&dest_path, anonymize)?;

    Ok(())
}