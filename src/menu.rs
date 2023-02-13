use crate::structs::GPTModel;
use crossterm::style::Stylize;

pub fn main_menu() {
    // clear terminal screen on startup and move cursor to top
    print!("\x1B[2J\x1B[1;1H");

    // Main menu
    println!("\n###############################");
    println!("\n#####   {}   ######", "[rust_gpt_cli]".blue());
    println!("\n###############################");
    println!("\n###   {}   ###", "Available Commands:".blue());
    println!("\n {} : {}  ", "exit".red(), "terminate program");
    println!("\n {} : {} (default: {})  ", "model".green(), "list OpenAI models", "text-davinci-003".magenta());
    println!("\n {} <{}> : {}  ", "model".green(), "id".magenta(), "change model");
    println!("\n {} : {}  ", "help".blue(), "list commands");
    println!("\n");
}

pub fn list_models(vm: &Vec<GPTModel>) {
    println!(
        "{:<17} | {: <44} | {: <30}", 
        format!("{}","id".green()), 
        format!("{}","name".magenta()), 
        format!("{}","owned_by".blue()),
    );
    println!("---|-------------------------------|----------------");
    for m in vm.iter() {
        println!("{:<17} | {: <44} | {: <30}", 
            format!("{}", format!("{}", m.id).green()), 
            format!("{}", m.name.clone().magenta()), 
            format!("{}", m.owned_by.clone().blue()),
        );
    }
    println!("\nType \"{} <{}>\" to change current model", "model".green(), "id".green());
}

pub fn change_model(l: &String, vm: &Vec<GPTModel>) -> String{
    if vm.is_empty() {
        println!("Please call {} command at least once before trying to change {}.", 
            "model".green(), 
            "model".magenta(),
        );
        return "".into();
    }
    let id: i32 = match l[5..].trim().parse::<i32>() {
        Ok(val) => val,
        _ => 0, 
    };

    if id == 0 {
        println!("Invalid {1} <{0}>. Please use an existing <{}> from the list of {}s.", "id".green(), "model".magenta());
        return "".into();
    }
    for m in vm.iter() {
        if m.id == id {
            return m.name.clone();
        }
    }
    println!("Invalid {1} <{0}>. Please use an existing <{}> from the list of {}s.", "id".green(), "model".magenta());
    return "".into();
}