use crossterm::{
    execute,
    terminal::{self, ClearType},
    Result,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::stdout;
use std::io::Read;
use std::{fs::File, io::Write};

fn clear_screen() -> Result<()> {
    let res: Result<_> = execute!(stdout(), terminal::Clear(ClearType::All));
    match res {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Failed to clear the screen: {:?}", e);
            Err(e)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    to_do_list: Vec<String>,
}

fn print_to_do(v: &Vec<String>) {
    if v.len() == 0 {
        println!("To do list is currently empty.");
    } else {
        for (ind, val) in v.iter().enumerate() {
            println!("({}) {}", ind, val);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut to_do_list: Vec<String> = match File::open("data.json") {
        Ok(mut file) => {
            let mut contents: String = String::new();
            file.read_to_string(&mut contents).unwrap();
            let data: Data = serde_json::from_str(&contents).unwrap();
            data.to_do_list
        }
        Err(_) => Vec::new(),
    };

    loop {
        clear_screen()?;
        println!("--- To Do List v1 ---");
        println!("---------------------");

        println!();

        print_to_do(&to_do_list);

        println!();

        println!("What would you like to do?");
        println!("(1) Add item (2) Remove item (3) Exit");

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).expect("Error");

        if buffer.trim() == "1" {
            clear_screen()?;
            print_to_do(&to_do_list);
            println!("What would you like to add?");
            println!();
            let mut new_item: String = String::new();
            io::stdin().read_line(&mut new_item).expect("Error");
            to_do_list.push(new_item.trim().to_string());
        } else if buffer.trim() == "2" {
            clear_screen()?;
            print_to_do(&to_do_list);
            println!();
            println!("What would you like to remove?");
            let mut remove_item: String = String::new();
            io::stdin().read_line(&mut remove_item).expect("Error");
            let remove_item_num: usize = remove_item.trim().parse().expect("Error");
            to_do_list.remove(remove_item_num);
        } else if buffer.trim() == "3" {
            println!("Goodbye!");
            // exit
            let data: Data = Data { to_do_list };
            let json: String = serde_json::to_string(&data).unwrap();
            let mut file: File = File::create("data.json").unwrap();
            file.write_all(json.as_bytes()).unwrap();
            break Ok(());
        } else {
            println!("Invalid input");
        }
    }
}
