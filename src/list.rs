use std::fs::File;

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

        let file_name = format!("{}/{}.txt",config.list_folder_path,list.name.clone());
        match File::create(file_name.clone()){
            Ok(file) => println!("{:?}", file),
            Err(_) => println!("Unable to create the file: '{}'", file_name)
        }

        return list;
    }
}