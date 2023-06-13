use std::ptr::write_volatile;
use std::{env, fs};
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::path::{Path, self};
use std::io::prelude::*;

use crate::list_item::*;
use crate::config::Config;

pub struct ToDoList{
    name: String,
    content: Option<String>,
}

fn get_directory_path(config: &Config,name: &str)->String{
    let temp_string = format!("{}",config.list_folder_path);
    let folder_path = Path::new(&temp_string);
    let directory_path = env::current_dir().expect("Expected to get path");

    
    //-----------------------------------------------------------------------------------------------------------------------------------------------------------
    //WARNING: WONT WORK FOR ANY FILE SYSTEM THAT CONTAINS NON UTF8 COMBATIBLE CHARACTERS SINCE NO CHECKS ARE IN PLACE RIGHT NOW AND PROPABLY NEVER WILL BE
    let temp_folder = folder_path.to_str().expect("Path should have been utf8 combatible");
    let temp_directory = directory_path.to_str().expect("Path should have been utf8 combatible");
    //-----------------------------------------------------------------------------------------------------------------------------------------------------------

    let final_path = format!("{}{}/{}.txt",temp_directory,temp_folder,name);

    final_path
}



impl ToDoList{
    pub fn create_list(name: &str,config: &Config)-> ToDoList{
        let list = ToDoList { 
            name: name.to_string() ,
            content:  None
        };

        let final_path = get_directory_path(config, name);
        // dbg!(&final_path);
        match File::create(&final_path){
            Ok(file) => {
                println!("List with name: {} created!",list.name);
                //println!("{:?}", file)
            },
            Err(_) => {
                println!("Unable to create the file");
                // dbg!("{}",&final_path);
            }
        }

        return list;
    }

    pub fn delete_list(name: &str,config: &Config){
        
        
        let final_path = get_directory_path(config, name);
        match fs::remove_file(final_path){
            Ok(_)=>{
                println!("List with name {} deleted!",name)
            }
            Err(e)=>{
                panic!("Error during deletion of file: {e}")
            }
        }


    }

    pub fn ls_lists(config: &Config){
        let temp_string = format!("{}",config.list_folder_path);
        let folder_path = Path::new(&temp_string);
        let directory_path = env::current_dir().expect("Expected to get path");
        
        //-----------------------------------------------------------------------------------------------------------------------------------------------------------
        //WARNING: WONT WORK FOR ANY FILE SYSTEM THAT CONTAINS NON UTF8 COMBATIBLE CHARACTERS SINCE NO CHECKS ARE IN PLACE RIGHT NOW AND PROPABLY NEVER WILL BE
        let temp_folder = folder_path.to_str().expect("Path should have been utf8 combatible");
        let temp_directory = directory_path.to_str().expect("Path should have been utf8 combatible");
        //-----------------------------------------------------------------------------------------------------------------------------------------------------------

        let final_path = format!("{}{}",temp_directory,temp_folder);
        let path = Path::new(final_path.as_str());
        // dbg!(final_path);
        
        
        println!("---------------------------------------------------------------------------------------------");
        println!("Available to-do lists are: ");
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                let temp_OsString = entry.file_name();
                let temp_str = temp_OsString.to_str().expect("Conversion should have worked");
                let current_list_name: &str = &temp_str[0..(&temp_str.len()-4)];
                println!("->{}", current_list_name);
                
            }
        }
        println!("---------------------------------------------------------------------------------------------");

    }

    pub fn add_item(list_name: &str,item: ListItem,config: &Config){
        
        
        let final_path = get_directory_path(config, list_name);

        let item_name = item.name.clone();
        let item_description: String;
        match &item.description {
            None=>{
                item_description = String::new();
            }
            Some(text)=>{
                item_description = String::from(text);
            }

        }
        let new_line:String;
        if item_description == ""{
            new_line = format!("-> {}",item_name);
        }
        else{
            new_line = format!("-> {} : {}",item_name,item_description);
        }
        // dbg!(&new_line);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(final_path)
            .unwrap();
        if let Err(e) = writeln!(file, "{}",new_line.clone()) {
            eprintln!("Couldn't write to file: {}", e);
        }


    }


    pub fn remove_item(list_name: &str,item_name: &str ,config: &Config){
        
        let final_path = get_directory_path(config, list_name);
        let list_contents_temp = fs::read_to_string(final_path.clone()).expect("Should have been able to read the file");
        let mut list_contents_lines:Vec<&str> = list_contents_temp.lines().collect();
        

        let mut line_counter = 0;
        
        let mut current_line: Vec<&str> = vec![];
        let mut temp_counter = 0;
        let mut found_item: bool = false;
        while line_counter < list_contents_lines.len(){
            current_line = list_contents_lines[line_counter].split_whitespace().collect();
            // dbg!(current_line[1],item_name);
            if current_line[1] == item_name{
                // dbg!("got here!");
                temp_counter = line_counter;
                found_item = true;
                break;
            };
            line_counter += 1;
        }
        if found_item{
            list_contents_lines.remove(temp_counter);
            let temp = format!("{}\n",list_contents_lines[list_contents_lines.len()-1]);
            let temp_index = list_contents_lines.len()-1;
            list_contents_lines[temp_index] = &temp;
            let new_list = list_contents_lines.join("\n");
            match File::create(final_path.clone()){
                Ok(mut file) => {
                    file.write(new_list.as_bytes());
                },
                Err(_) => {
                    println!("Unable to open/create the file");
                    // dbg!("{}",&final_path);
                }
            }

        }

        
    }   

    pub fn view_list(list_name: &str,config: &Config){
        let final_path = get_directory_path(config, list_name);
        let file_content = fs::read_to_string(final_path).expect("Should have been able to read the file");

        println!("{}",file_content);
    }

}