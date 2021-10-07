mod utils;
mod controller;
mod model;
mod cli;

use std::{io::{Write, stdin},env};
use serde::{Serialize, Deserialize};
use cli::*;

#[derive(Serialize, Deserialize)]
struct User {
    id: String,
    pass: String
}
impl ::std::default::Default for User {
    fn default() -> Self { Self {id: "".into(), pass: "".into() } }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut id = String::from("");
    let mut pass = String::from("");
    if args.len() > 1 && args[1].eq("--login") {
        print!("PoliMi id: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut id).unwrap();
        print!("Password: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut pass).unwrap();
        print!("\x1B[2J\x1B[1;1H");
        let my_cfg = User {id: id.clone(), pass: pass.clone()};
        confy::store("webeep",my_cfg).expect("Error in creating the config file!");
    }
    let cfg : User = confy::load("webeep").expect("Error in reading the config file!");
    if cfg.id.eq("") {
        println!("You only have to enter your credentials on the first launch!");
        print!("PoliMi id: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut id).unwrap();
        print!("Password: ");
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut pass).unwrap();
        print!("\x1B[2J\x1B[1;1H");
        let my_cfg = User {id: id.clone(), pass: pass.clone()};
        confy::store("webeep",my_cfg).expect("Error in creating the config file!");
    }
    else {
        id = cfg.id;
        pass = cfg.pass;
    }
    let mut controller = controller::Controller::new(id.trim(),pass.trim());
    controller.start();
    let mut usr_input = String::new();
    let mut curr_pos = Vec::new();
    curr_pos.push(String::from("/"));
    print!("\x1B[2J\x1B[1;1H");
    while !usr_input.eq(&String::from("exit\n")) {
        usr_input.clear();
        cli::print_logo();
        stdin().read_line(&mut usr_input).expect("msg"); 

        let mut cmd_iter = usr_input.split_ascii_whitespace();
        let cmd_un = cmd_iter.next();
        let cmd;
        if cmd_un != None {
            cmd = cmd_un.unwrap();
        }
        else {
            continue
        }

        if cmd.contains("ls") {
            let param = cmd_iter.next();
            if param != None {
                ls_l(&controller, &curr_pos);
            }
            else {
                ls(&controller, &curr_pos);
            }
        }
        else if cmd.contains("cd") {
            let param = cmd_iter.next();
            if param != None {
                if param.unwrap().eq("..") {
                    cd(param.unwrap(), &mut curr_pos);
                    continue
                }
                let p = param.unwrap().parse::<usize>();
                if p.is_err() {
                    continue
                }
                let real_p = p.unwrap();
                if curr_pos.len() == 1 {
                    if real_p < controller.courses.len() {
                        cd(param.unwrap(), &mut curr_pos);
                    }
                }
                else if curr_pos.len() == 2 {
                    let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
                    let course = &controller.courses[index];
                    if real_p < course.dirs.len() {
                        cd(param.unwrap(), &mut curr_pos);
                    }
                }
            }
        }
        else if cmd.contains("pwd") {
            pwd(&controller, &curr_pos);
        }
        else if cmd.contains("get") {
            let filename = cmd_iter.next();
            let mut dest = cmd_iter.next();
            if filename != None && dest != None { 
                if !filename.unwrap().eq("-all") {
                    let mut indeces = Vec::new();
                    indeces.push(filename.unwrap());
                    let mut one_param = false;
                    let mut d_not_found = false;
                    let mut temp = "";
                    if !dest.unwrap().eq("-d") {
                        indeces.push(dest.unwrap());
                        temp = cmd_iter.next().unwrap();
                        d_not_found = false;
                        while !one_param && !temp.eq("-d") {
                            indeces.push(temp);
                            let next = cmd_iter.next();
                            if next == None {
                                d_not_found = true;
                                break;
                            }
                            temp = next.unwrap();
                        }
                    }
                    else {
                        one_param = true;
                    }
                    
                    if d_not_found && !one_param{
                        println!("Missing destination path...");
                        continue;
                    }
                    if temp.eq("-d") || one_param {
                        dest = cmd_iter.next();
                    }
                    if curr_pos.len() == 2 {
                        let file_dest = dest.unwrap();
                        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
                        let course = &controller.courses[index].clone();
                        let files = &course.files;
                        for filename in indeces {
                            let file_index = filename.parse::<usize>();
                            if file_index.is_err() {
                                println!("Incorrect file index!");
                                continue
                            }
                            let corr_file_index = file_index.unwrap();
                            let file = &files[corr_file_index];
                            controller.download_file(file.clone(), String::from(String::from(file_dest).trim()));
                        }
                    }
                    else if curr_pos.len() == 3 {
                        let file_dest = dest.unwrap();
                        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
                        let course = &controller.courses[index];
                        let index2:usize = curr_pos.get(2).unwrap().parse().unwrap();
                        let dirs = course.dirs_with_files[index2].clone();
                        let files = dirs.1;
                        for filename in indeces {
                            let file_index = filename.parse::<usize>();
                            if file_index.is_err() {
                                println!("Incorrect file index!");
                                continue
                            }
                            let corr_file_index = file_index.unwrap();
                            let file = &files[corr_file_index];
                            controller.download_file(file.clone(), String::from(String::from(file_dest).trim()));
                        }
                    }
                }
                else if filename.unwrap().eq("-all") {
                    if curr_pos.len() == 2 {
                        let file_dest = dest.unwrap();
                        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
                        let course = &controller.courses[index];
                        let files = &course.files;
                        for file in files.clone() {
                            controller.download_file(file.clone(), String::from(String::from(file_dest).trim()));
                        }
                    }
                    else if curr_pos.len() == 3 {
                        let file_dest = dest.unwrap();
                        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
                        let course = &controller.courses[index];
                        let index2:usize = curr_pos.get(2).unwrap().parse().unwrap();
                        let dirs = course.dirs_with_files[index2].clone();
                        let files = dirs.1;
                        for file in files {
                            controller.download_file(file.clone(), String::from(String::from(file_dest).trim()));
                        }
                    }
                }
            }
        }
        else if cmd.contains("clear") {
            print!("\x1B[2J\x1B[1;1H");
        }
        else if !cmd.contains("exit") {
            println!("Invalid command!")
        }
    }
}


