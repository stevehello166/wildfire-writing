use std::fs::File;
use fltk::dialog;

use crate::OPERATING_SYSTEM;

/*
This module is a simple utility allowing me to easily create files and associated dialogs

KNOWN ISSUES
code panics if cancel is pressed instead of inputting values
*/

pub fn create_file(file_type: &str) {
    let file_name = dialog::input(100, 100, "Input Name Of File", "Error");
    let mut folder_dir =  dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
    folder_dir.show();

    //Folder_dir panics when cancel is pressed on the open folder dialog.
    //Makes folder_dir and file_name the same type of string.
    let folder_dir = folder_dir
        .filename()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut file_name = file_name.unwrap();
    let file_name = file_name.as_mut_str();



    let folder_dir = folder_dir + "/" + file_name + file_type;

    println!("{}", folder_dir);

    let mut _temp = File::create(folder_dir) 
        .expect("Error encountered while creating file!");
}
