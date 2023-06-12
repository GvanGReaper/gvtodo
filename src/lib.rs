use std::{error::Error, vec};
use print_messages::*;
use list::*;
use config::*;

pub mod print_messages;
pub mod list;
pub mod config;


pub fn run(args: Vec<String>)-> Result<(),Box<dyn Error>>{    
    let config = create_config(args);


    if config.flag == "none"{
        command_not_recognized();
    }
    else if config.flag == "help"{
        print_help();
    }
    else if config.flag == "add"{

    }
    else if config.flag == "remove"{

    }
    else if config.flag == "create"{
        let mut list_name: String = String::new();
        match &config.vars{
            None=> panic!("EXPECTED VARIABLE [list_name],got NONE"),
            Some(vars)=>{
                list_name = vars[0].clone();
            }
        }
        ToDoList::create_list(&list_name, &config);
    }

    Ok(())
}

