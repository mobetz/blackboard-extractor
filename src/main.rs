// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env::current_dir, error::Error};
use slint::{ComponentHandle, Model, SharedString};

mod process;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;


    ui.global::<FileDialog>().on_startBrowseFile(|model| {
        //TODO: async pain

        
        let mut dialog = rfd::FileDialog::new()
            .set_directory(current_dir().unwrap());

        for ext in model.iter()  {
            let ftypes: Vec<SharedString> = ext.0.iter().collect();
            dialog = dialog.add_filter(ext.1, &ftypes);
        }
    
    
        let selected_file = dialog.pick_file()
            .unwrap_or_default()
            .to_str().unwrap_or("").to_string();   
        
        SharedString::from(selected_file)
    });



    ui.global::<FileDialog>().on_startBrowseFolder(|| {
        //TODO: async pain

        
        let dialog = rfd::FileDialog::new()
            .set_directory(current_dir().unwrap());
    
        let selected_folder = dialog.pick_folder()
            .unwrap_or_default()
            .to_str().unwrap_or("").to_string();   
        
        SharedString::from(selected_folder)
    });


    ui.on_submitted({
        let ui_handler = ui.as_weak();
        move || { 
            let ui = ui_handler.upgrade().unwrap();
            let zip_loc = ui.get_zip_location();
            let dest_loc = ui.get_dest_location();
            let anonymized = ui.get_anonymize();
            
            process::process_blackboard_zip(&zip_loc, &dest_loc, anonymized).unwrap();
        }
    });


    ui.run()?;

    Ok(())
}

