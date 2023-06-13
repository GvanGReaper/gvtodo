pub struct ListItem{
    pub name: String,
    pub description: Option<String>,
}

impl ListItem{
    pub fn create_item(item_name: &str,item_description: Option<String>)->ListItem{
        ListItem { 
            name: item_name.to_string(),
            description: item_description  
        }
    }
}