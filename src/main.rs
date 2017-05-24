/* 
    Rake in Rust.
    Ruby build tool implemented in Rust.
    Copyright 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

mod variable;
mod task;
extern crate clioptions;
extern crate regex;
use variable::Variable;
use task::Task;
use clioptions::CliOptions;
use regex::Regex;
use std::io::Read;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::{Command, exit};

fn parse_vars_in_task(task: &Task, vars: &Vec<Variable>) -> Task {
    let mut split = task.get_params().split(" ");
    let params: Vec<&str> = split.collect();
    let mut pparams: Vec<String> = Vec::new();
    for param in params {
        let p = Regex::new(&format!("#{}(.*){}", 
        regex::escape("{"), regex::escape("}"))).unwrap();
        if p.is_match(param) {
            for cap in p.captures_iter(param) {
                for var in vars {
                    if var.get_key() == cap[1].to_owned() {
                        pparams.push(var.get_value());
                    }
                }
            }
        } else {
            pparams.push(param.to_owned());
        }
    }
    Task::new(&task.get_name(), &task.get_command(), &pparams.join(" "))
}

fn invoke_rakefile(program: &str, rakefile: &str, stasks: &Vec<String>) {
    let mut rf = String::new();
    let mut name = String::new();
    let mut command = String::new();
    let mut params = String::new();
    let mut vars: Vec<Variable> = Vec::new();
    let mut tasks: Vec<Task> = Vec::new();
    let mut file = File::open(rakefile).unwrap();
    let _ = file.read_to_string(&mut rf);
    let mut split = rf.split("\n");
    let lines: Vec<&str> = split.collect();
    for l in lines {
        let mut p = Regex::new("^#").unwrap();
        if p.is_match(&l) {
            continue;
        }
        p = Regex::new("(.*)=.*\"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            vars.push(Variable::new(&cap[1].trim(), &cap[2].trim()));
        }
        p = Regex::new("task :(.*) do").unwrap();
        for cap in p.captures_iter(&l) {
            name = cap[1].to_owned();
        }
        p = Regex::new("(puts) \"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            tasks.push(Task::new(&name, &command, &params));
        }
        p = Regex::new("(sh) \"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            tasks.push(Task::new(&name, &command, &params));
        }
        p = Regex::new("(File.delete).*\"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            tasks.push(Task::new(&name, &command, &params));
        }
    }

    let mut ptasks: Vec<Task> = Vec::new();
    for task in &tasks {
        let ptask = parse_vars_in_task(&task, &vars);
        ptasks.push(ptask);
    }

    // --------------------------------
    //println!("Vars: {:?}", vars);
    //println!("Tasks: {:#?}", ptasks);
    // --------------------------------

    let mut matched = false;
    let mut qtask = String::new();
    for stask in stasks {
        for task in &ptasks {
            qtask = stask.to_owned();
            if task.get_name() == stask {
                matched = true;
                match task.get_command() {
                    "puts" => println!("{}", task.get_params()),
                    "sh" => {
                        println!("{}", task.get_params());
                        let mut split = task.get_params().split(" ");
                        let mut args: Vec<&str> = split.collect();
                        let cmd = args[0]; args.remove(0);
                        let output = Command::new(&cmd)
                        .args(&args)
                        .output()
                        .expect("failed to execute process");
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    },
                    "File.delete" => {
                        let file = &task.get_params();
                        if Path::new(file).exists() {
                            fs::remove_file(file).unwrap()
                        }
                    },
                    _ => {},
                }
            }
        }
        if !matched {
            throw_no_task_failure(&program, &qtask);
        }
    }
    exit(0);
}

fn parse_tasks(program: &str, tasks: Vec<String>) -> Vec<String> {
    let mut ptasks: Vec<String> = Vec::new();
    for task in tasks {
        let p = Regex::new(&format!("{}|akefile.*", program)).unwrap();
        if !p.is_match(&task) {
            ptasks.push(task.to_owned());
        }
    }
    ptasks
}

fn throw_no_task_failure(program: &str, task: &str) {
    println!("{} aborted!", program);
    println!("Don't know how to build task '{}'\n", task);
    exit(-1);   
}

fn throw_not_found_failure(program: &str, rakefiles: &Vec<&str>) {
    println!("{} aborted!", program);
    println!("No Rakefile found (looking for {})\n", rakefiles.join(", "));
    exit(-1);
}

fn display_version() {
    println!("rake in rust, version 0.1.0");
    exit(0);
}

fn display_error(program: &str, err: &str) {
    println!("Error: {}.\n", err);
    display_usage(program, -1);
}

fn display_usage(program: &str, code: i32) {
    println!("Rake implementation in Rust.");
    exit(code);
}

fn main() {
    let cli = CliOptions::new("rrake");
    let program = cli.get_program();
    // ------------------------------------------------------------------------
    let rakefiles = vec!["rakefile", "Rakefile", "rakefile.rb", "Rakefile.rb"];
    // ------------------------------------------------------------------------
    //let mut verbose = true;
    let mut tasks: Vec<String> = Vec::new();
    let mut srakefile = String::new();

    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                "-f" | "--rakefile" => srakefile = cli.next_argument(i),
                _ => tasks.push(a.to_owned()),
            }
        }
    }

    let mut tasks = parse_tasks(&program, tasks);
    if tasks.len() == 0 {
        tasks.push("default".to_owned());
    }
    if !srakefile.is_empty() {
        if Path::new(&srakefile).exists() {
            invoke_rakefile(&program, &srakefile, &tasks);
        }
    }

    for rakefile in &rakefiles {
        if Path::new(&rakefile).exists() {
            invoke_rakefile(&program, &rakefile, &tasks);
        }
    }
    throw_not_found_failure(&program, &rakefiles);
}
