pub fn command_not_recognized(){
    println!("Command given is not recognized?Did you perhaps make a typing error?");
    println!("Pass the -h flag for a complete list of all available commands");
}


pub fn print_help(){
    println!("List of available commands:");
    println!("---------------------------------------------------------------------------------------------");
    println!("help/-h: shows help");
    println!("---------------------------------------------------------------------------------------------");
    println!("Item related commands:");
    println!("1.add/-a [list_name][item_name(must be unique)][item_description(optional)]: add item to to do list");
    println!("2.remove/-r [list_name][item_name]: remove item from to do list");
    println!("---------------------------------------------------------------------------------------------");
    println!("List related commands:");    
    println!("1.create/-c [list_name]: creates new to do list");
    println!("2.view/-v [list name]: allows you to view a list");
    println!("3.list/-ls: shows all created lists");
    println!("---------------------------------------------------------------------------------------------");
}