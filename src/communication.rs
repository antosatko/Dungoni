use colored::Colorize;

use crate::Texts;

pub fn load(source: &str) -> String {
    Texts::get(source)
        .unwrap()
        .data
        .iter()
        .map(|x| *x as char)
        .collect::<String>()
}

pub fn print(source: &str) {
    println!("{}", load(source));
}

pub fn print_colored(source: &str, color: Colors) {
    let text = load(source);
    match color {
        Colors::Red => println!("{}", text.red()),
        Colors::Green => println!("{}", text.green()),
        Colors::Blue => println!("{}", text.blue()),
        Colors::Yellow => println!("{}", text.yellow()),
        Colors::Magenta => println!("{}", text.magenta()),
        Colors::Cyan => println!("{}", text.cyan()),
        Colors::White => println!("{}", text.white()),
        Colors::Black => println!("{}", text.black()),
    }
}

pub fn wrong_input() {
    println!("{}", "I'm sorry, I didn't understand your command.".red());
}

pub fn get_input(can_help: bool) -> String {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "help" && can_help {
            get_help();
            input = String::new();
            continue;
        } else {
            break input;
        }
    }
}

/// this function will return the index of the command in the commands vector
/// if the command is not found, it will return None
/// rules:
/// 1. the command does not have to be the full word
/// 2. the command is case insensitive
/// 3. the command can be a number, if it's in the range of the vector
/// 4. the command can not be empty
/// 5. the command must be unique sequence of characters from list of commands
pub fn match_command(command: &str, commands: &Vec<String>) -> Option<usize> {
    if command.len() == 0 {
        return None;
    }
    let mut idx = None;
    for (index, item) in commands.iter().enumerate() {
        if item.to_lowercase().starts_with(&command) {
            if idx.is_some() {
                return None;
            }
            idx = Some(index);
        }
    }
    idx
}

pub fn is_exit(input: &str) -> bool {
    match_command(input, &vec!["exit".to_string(), "quit".to_string()]).is_some()
}

pub fn get_input_with_exit(can_help: bool) -> Option<String> {
    let mut input = String::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "help" && can_help {
            get_help();
            input = String::new();
            continue;
        } else if is_exit(input.as_str()) {
            println!("Exiting...");
            return None;
        } else {
            break Some(input);
        }
    }
}

pub fn get_help() {
    print_colored("help.txt", Colors::Yellow);
    // get all files in the texts/help folder
    let mut commands = Vec::new();
    for file in Texts::iter().filter(|x| x.starts_with("help/")) {
        commands.push(file.replace("help/", "").replace(".txt", ""));
    }
    match many_commands_with_exit(&commands, false) {
        Some(choice) => {
            print(&format!("help/{}", commands[choice]));
            get_help();
        }
        None => (),
    }
}
pub fn many_commands(commands: &Vec<String>, can_help: bool) -> usize {
    let mut result = String::new();
    for command in commands.iter().enumerate() {
        result.push_str(&format!(
            "{}. {}\n",
            command.0,
            command.1.as_str().magenta()
        ));
    }
    println!("{}---------------------", result);
    let mut input = get_input(can_help).to_lowercase();
    while match_command(&input, commands).is_none() {
        // if the input is a number and it's in the range of the vector
        if input.parse::<usize>().is_ok()
            && input.parse::<usize>().unwrap() < commands.len() as usize
        {
            return input.parse::<usize>().unwrap();
        }
        wrong_input();
        input = get_input(can_help).to_lowercase();
    }
    // return the index of the command in the vector
    match_command(&input, commands).unwrap()
}

pub fn many_commands_with_exit(commands: &Vec<String>, can_help: bool) -> Option<usize> {
    let mut result = String::new();
    for command in commands.iter().enumerate() {
        result.push_str(&format!(
            "{}. {}\n",
            command.0,
            command.1.as_str().magenta()
        ));
    }
    println!("{}---------------------", result);
    let mut input = get_input_with_exit(can_help);
    while input.is_some() && match_command(&input.as_ref().unwrap(), commands).is_none() {
        // if the input is a number and it's in the range of the vector
        if input.as_ref().unwrap().parse::<usize>().is_ok()
            && input.as_ref().unwrap().parse::<usize>().unwrap() < commands.len() as usize
        {
            return input.as_ref().unwrap().parse::<usize>().ok();
        }
        wrong_input();
        input = get_input_with_exit(can_help);
    }
    // return the index of the command in the vector
    if input.is_some() {
        Some(match_command(&input.as_ref().unwrap(), commands).unwrap())
    } else {
        None
    }
}

pub fn many_commands_with_description(commands: &Vec<(String, String)>, can_help: bool) -> usize {
    let mut result = String::new();
    for (command, description) in commands.iter().enumerate() {
        result.push_str(&format!("{}. {} - {}\n", command, description.0.magenta(), description.1));
    }
    let mut comms = Vec::new();
    for i in 0..commands.len() {
        comms.push(commands[i].0.clone());
    }
    println!("{}---------------------", result);
    let mut input = get_input(can_help).to_lowercase();
    while match_command(&input, &comms).is_none() {
        // if the input is a number and it's in the range of the vector
        if input.parse::<usize>().is_ok()
            && input.parse::<usize>().unwrap() < commands.len() as usize
        {
            return input.parse::<usize>().unwrap();
        }
        wrong_input();
        input = get_input(can_help).to_lowercase();
    }
    // return the index of the command in the vector
    match_command(&input, &comms).unwrap()
}

pub fn yesno() -> bool {
    println!("Correct? ({}/{})", "yes", "no".yellow());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    match match_command(&input, &vec!["yes".to_string(), "no".to_string()]) {
        Some(i) => i == 0,
        None => false,
    }
}

#[allow(dead_code)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    Black,
}
