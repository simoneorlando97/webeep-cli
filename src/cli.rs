use std::io::{Write, stdout};

use crate::controller::{Controller};

pub fn print_logo() {
    print!("webeep> ");
    stdout().flush().expect("msg");
}

pub fn ls(controller : &Controller, curr_pos : &Vec<String>) {
    if curr_pos.len() == 1 {
        let mut i = 0;
        for course in &controller.courses {
            println!("[{}] {}",i,course.name);
            i = i+1;
        }
    }
    else if curr_pos.len() == 2 {
        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
        let course = &controller.courses[index];
        println!("---- [Files] ----");
        let mut i = 0;
        for file in &course.files {
            println!("[{}] {}",i,file.0);
            i = i + 1;
        }
        println!("---- [Dirs] ----");
        let mut i = 0;
        for dir in &course.dirs {
            println!("[{}] {}",i,dir.0);
            i = i+1;
        }
    }
    else if curr_pos.len() == 3 {
        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
        let course = &controller.courses[index];
        let index2:usize = curr_pos.get(2).unwrap().parse().unwrap();
        let dirs = course.dirs_with_files[index2].clone();
        let mut i = 0;
        for dir in dirs.1 {
            println!("[{}] {}",i, dir.0);
            i = i+1;
        }
    }
}

pub fn ls_l(controller : &Controller, curr_pos : &Vec<String>) {
    if curr_pos.len() == 1 {
        let mut i = 0;
        for course in &controller.courses {
            println!("[{}] {} --> {}",i,course.name, course.resource_page_url);
            i = i+1;
        }
    }
    else if curr_pos.len() == 2 {
        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
        let course = &controller.courses[index];
        println!("---- [Files] ----");
        let mut i = 0;
        for file in &course.files {
            println!("[{}] {} --> {}",i,file.0,file.1);
            i = i + 1;
        }
        println!("---- [Dirs] ----");
        let mut i = 0;
        for dir in &course.dirs {
            println!("[{}] {} --> {}",i,dir.0,dir.1);
            i = i+1;
        }
    }
    else if curr_pos.len() == 3 {
        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
        let course = &controller.courses[index];
        let index2:usize = curr_pos.get(2).unwrap().parse().unwrap();
        let dirs = course.dirs_with_files[index2].clone();
        let mut i = 0;
        for dir in dirs.1 {
            println!("[{}] {} --> {}", i,dir.0, dir.1);
            i = i + 1;
        }
    }
}

pub fn cd(new_loc : &str, curr_pos : &mut Vec<String>) {
    if curr_pos.len() == 1 {
        if !new_loc.eq("..") {
            curr_pos.push(String::from(new_loc));
        }
    }
    else if curr_pos.len() == 2 {
        if !new_loc.eq("..") {
            curr_pos.push(String::from(new_loc));
        }
        else {
            curr_pos.pop();
        }
    }
    else if curr_pos.len() == 3{
        if !new_loc.eq("..") {
            curr_pos.push(String::from(new_loc));
        }
        else {
            curr_pos.pop();
        }
    }
}

pub fn pwd(controller : &Controller, curr_pos : &Vec<String>) {
    if curr_pos.len() == 1 {
        println!("/");
    }
    else if curr_pos.len() == 2{
        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
        let course = &controller.courses[index];
        println!("{}",course.name);
    }
    else if curr_pos.len() == 3 {
        let index:usize = curr_pos.get(1).unwrap().parse().unwrap();
        let course = &controller.courses[index];
        let index2:usize = curr_pos.get(2).unwrap().parse().unwrap();
        let dirs = course.dirs_with_files[index2].clone();
        println!("{}", dirs.0);
    }
}