use std::env;
use std::fmt::format;
use std::fs::File;
use std::path::{Path, self};


use crate::config::Config;

pub struct ToDoList{
    name: String,
    content: Option<String>,
}

impl ToDoList{
    pub fn create_list(name: &str,config: &Config)-> ToDoList{
        let list = ToDoList { 
            name: name.to_string() ,
            content:  None
        };
        let temp_string = format!("{}",config.list_folder_path);
        let folder_path = Path::new(&temp_string);
        let directory_path = env::current_dir().expect("Expected to get path");

        //-----------------------------------------------------------------------------------------------------------------------------------------------------------
        //WARNING: WONT WORK FOR ANY FILE SYSTEM THAT CONTAINS NON UTF8 COMBATIBLE CHARACTERS SINCE NO CHECKS ARE IN PLACE RIGHT NOW AND PROPABLY NEVER WILL BE
        let temp_folder = folder_path.to_str().expect("Path should have been utf8 combatible");
        let temp_directory = directory_path.to_str().expect("Path should have been utf8 combatible");
        //-----------------------------------------------------------------------------------------------------------------------------------------------------------


        let final_path = format!("{}{}/{}",temp_directory,temp_folder,list.name);
        // dbg!(&final_path);
        match File::create(&final_path){
            Ok(file) => println!("{:?}", file),
            Err(_) => {
                println!("Unable to create the file");
                // dbg!("{}",&final_path);
            }
        }

        return list;
    }
}