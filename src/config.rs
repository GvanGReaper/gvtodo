use std::{fs, task::Context};


pub struct Config{
    pub curent_running_folder: String,
    pub list_folder_path: String,
    pub flag: String,
    pub vars: Option<Vec<String>>,
}

pub fn create_config(args: Vec<String>)-> Config{
    if args.len() < 2{
        panic!("No flag given!If you wanna see the list of available commands,try passing the -h/help flag")        
    }
    else{
        let current_file = args[0].clone();
        let current_flag = recognise_flag(&args);
        let optional_vars: Option<Vec<String>> = get_extra_vars(&args);
        
        let temp = fs::read_to_string("/mnt/hdd/programming/rust/gvToDo!/gvtodo/gvtodo.conf").expect("Should have been able to read the file");
        let contents:Vec<&str> = temp.split_whitespace().collect();
        
        let config_file_settings: Vec<String> = read_config_file(contents.clone());
        let config = Config{
            curent_running_folder: current_file,
            flag: current_flag,
            vars: optional_vars,
            list_folder_path: config_file_settings[0].clone(),
        };
        config
    }
}

fn read_config_file(content: Vec<&str>)->Vec<String>{
    let mut result: Vec<String> = vec![];
    let mut counter = 0;
    while counter < content.len(){
        if content[counter] == "list_folder_path"{
            counter += 2;
            result.push(content[counter].to_string());
        } 
        counter += 1;
    }    
    result
}


fn recognise_flag(args: &Vec<String>)-> String{
    let mut result: String = String::from("none");
    let mut flag_arg: &String = &args[1];
    if flag_arg == "-h" || flag_arg == "help"{
        result = "help".to_string();
    }
    else if flag_arg == "-a" || flag_arg == "add"{
        result = "add".to_string();
    }
    else if flag_arg == "-r" || flag_arg == "remove"{
        result = "remove".to_string();
    }
    else if flag_arg == "-c" || flag_arg == "create"{
        result = "create".to_string();
    }
    result
}


fn get_extra_vars(args: &Vec<String>)-> Option<Vec<String>>{
    let mut vars: Vec<String> = vec![];
    let mut counter = 2;
    if args.len() < 2{
        return None;
    }
    else{
        while counter < args.len(){
            vars.push(args[counter].to_string());
            counter += 1;
        }
        return Some(vars)
    }
}