use std::{error::Error, vec,process::exit};
use print_messages::*;
use list::*;
use config::*;
use list_item::*;

pub mod print_messages;
pub mod list;
pub mod config;
pub mod list_item;

pub fn run(args: Vec<String>)-> Result<(),Box<dyn Error>>{    
    let config = create_config(args);


    if config.flag == "none"{
        command_not_recognized();
    }
    else if config.flag == "help"{
        print_help();
    }
    else if config.flag == "add"{
        let mut list_name: String = String::new();
        let mut item_name: String = String::new();
        let mut item_description: Option<String> = None;
        
        match &config.vars{
            None=> panic!("EXPECTED VARIABLES [list_name][item_name][item_text(optional)],got NONE"),
            Some(vars)=>{
                if vars.len() < 2{
                    println!("This command takes atleast the 2 arguments [list_name][item_name],got {} arguments instead.",vars.len());
                    std::process::exit(1);
                }
                list_name = vars[0].clone();
                item_name = vars[1].clone();
                if vars.len() == 3{
                    item_description = Some(vars[2].clone());
                }
            }
        }

        let item = ListItem::create_item(&item_name, item_description);
        ToDoList::add_item(&list_name, item, &config);
    }
    else if config.flag == "remove"{
        
        let mut list_name: String = String::new();
        let mut item_name: String = String::new();
        
        match &config.vars{
            
            None=> panic!("EXPECTED VARIABLES [list_name][item_name],got NONE"),
            Some(vars)=>{
                // dbg!(&config.vars);
                if vars.len() != 2{
                    println!("This command takes the 2 arguments [list_name][item_name],got {} arguments instead.",vars.len());
                    std::process::exit(1);
                }
                list_name = vars[0].clone();
                item_name = vars[1].clone();
            }
        }
        ToDoList::remove_item(&list_name, &item_name, &config);

    }
    else if config.flag == "create"{
        let mut list_name: String = String::new();
        match &config.vars{
            None=> panic!("EXPECTED VARIABLE [list_name],got NONE"),
            Some(vars)=>{
                if vars.len() > 1{
                    println!("This command takes the 1 argument [list_name],got {} arguments instead.",vars.len());
                    std::process::exit(1);
                }
                list_name = vars[0].clone();
            }
        }
        ToDoList::create_list(&list_name, &config);
    }
    else if config.flag == "delete"{
        let mut list_name: String = String::new();
        match &config.vars{
            None=> panic!("EXPECTED VARIABLE [list_name],got NONE"),
            Some(vars)=>{
                if vars.len() > 1{
                    println!("This command takes 1 argument [list_name],got {} arguments instead.",vars.len());
                    std::process::exit(1);
                }
                list_name = vars[0].clone();
            }
        }
        ToDoList::delete_list(&list_name, &config);   
    }
    else if config.flag == "list"{
        ToDoList::ls_lists(&config);
    }
    else if config.flag == "view"{
        let mut list_name: String = String::new();
        match &config.vars{
            None=> panic!("EXPECTED VARIABLE [list_name],got NONE"),
            Some(vars)=>{
                if vars.len() > 1{
                    println!("This command takes the 1 argument [list_name],got {} arguments instead.",vars.len());
                    std::process::exit(1);
                }
                list_name = vars[0].clone();
            }
        }
        ToDoList::view_list(&list_name, &config);
    }

    Ok(())
}

