/* 
    Rake in Rust.
    Ruby build tool implementation.
    Copyright 2017 Sam Saint-Pettersen.

    Released under the MIT License.
*/

mod variable;
mod task;
mod rstruct;
extern crate clioptions;
extern crate regex;
extern crate os_type;
use variable::Variable;
use task::Task;
use rstruct::Struct;
use clioptions::CliOptions;
use regex::Regex;
use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::Path;
use std::thread;
use std::env;
use std::process::{Command, Stdio, exit};

struct Options {
    verbose: bool,
    exit_codes: bool,
    ignore: bool,
}

fn get_struct_fields(cfields: &str) -> Vec<String> {
    let split = cfields.split(",");
    let fields: Vec<&str> = split.collect();
    let mut ffields: Vec<String> = Vec::new();
    for (i, f) in fields.iter().enumerate() {
        let ff: String;
        if i == 0 {
            ff = format!("{}", &f[1..].trim());
        } else if i == fields.len() - 1 {
            ff = format!("{}", &f[0..f.len() - 1].trim());
        } else {
            ff = format!("{}", f.trim());
        }
        if !ff.is_empty() {
            ffields.push(ff.to_owned());
        }
    }
    ffields
}

fn get_struct_values(cvalues: &str) -> Vec<String> {
    let split = cvalues.split(",");
    let values: Vec<&str> = split.collect();
    let mut fvalues: Vec<String> = Vec::new();
    for (i, v) in values.iter().enumerate() {
        let vv: String;
        if i == 0 {
            vv = format!("{}", &v[1..].trim());
        } else if i == values.len() - 1 {
            vv = format!("{}", &v[0..v.len() - 1].trim());
        } else {
            vv = format!("{}", v.trim());
        }
        if !vv.is_empty() {
            fvalues.push(vv.to_owned());
        }
    }
    fvalues
}

fn validate_rakefile(rakefile: &str) -> bool {
    let mut valid = false;
    let mut rf = String::new();
    let mut file = File::open(&rakefile).unwrap();
    let _ = file.read_to_string(&mut rf);
    let p = Regex::new("task .* do").unwrap();
    if p.is_match(&rf) {
        valid = true;
    }
    valid
}

fn validate_extension(rakefile: &str) -> bool {
    let mut valid = false;
    let p = Regex::new("Rakefile|rakefile|^.rb$").unwrap();
    if p.is_match(&rakefile) {
        valid = true;
    }
    valid
}

fn parse_unit(unit: &str) -> i32 {
    let n = unit.parse::<i32>().ok();
    let unit = match n {
        Some(unit) => unit as i32,
        None => 0 as i32,
    };
    unit
}

fn get_os() -> String {
    let os = os_type::current_platform();
    format!("{:?}", os.os_type)
}

fn process_vars(rvars: Vec<String>, mut vars: Vec<Variable>) -> Vec<Variable> {
    let mut prvars: Vec<String> = Vec::new();
    let mut pvars: Vec<Variable> = Vec::new();
    vars.reverse();
    for (i, rvar) in rvars.iter().enumerate() {
        if !prvars.contains(rvar) {
            prvars.push(rvar.clone());
            pvars.push(vars[i].clone());
        }
    }
    pvars
}

fn process_struct_vars(structs: &Vec<Struct>, mut pvars: Vec<Variable>) -> Vec<Variable> {
    for v in &mut pvars {
        for s in structs {
            if s.get_variable() == v.get_key() {
                v.set_value(&s.to_json());
            }
        }
    }
    pvars
}

fn parse_vars_in_task(task: &Task, vars: &Vec<Variable>) -> Task {
    let split = task.get_params().split(" ");
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
    Task::new(&task.get_name(), &task.get_depends(), 
    &task.get_command(), &pparams.join(" "), task.get_line())
}

fn invoke_rakefile(program: &str, rakefile: &str, stasks: &Vec<String>, opts: &Options) {
    let mut rf = String::new();
    let mut name = String::new();
    let mut depends = String::new();
    let mut command: String;
    let mut params: String;
    let mut vars: Vec<Variable> = Vec::new();
    let mut structs: Vec<Struct> = Vec::new();
    let mut rvars: Vec<String> = Vec::new();
    let mut rstructs: Vec<String> = Vec::new();
    let mut tasks: Vec<Task> = Vec::new();
    let mut file = File::open(rakefile).unwrap();
    let _ = file.read_to_string(&mut rf);
    let split = rf.split("\n");
    let lines: Vec<&str> = split.collect();
    let mut in_block = false;
    let mut l_puts = false;
    for (i, l) in lines.iter().enumerate() {
        let mut p = Regex::new("^#").unwrap();
        if p.is_match(&l) {
            continue;
        }
        p = Regex::new("^end").unwrap();
        if p.is_match(&l) {
            in_block = false;
            continue;
        }
        p = Regex::new("(.*)=.*Struct.new(((.*)))").unwrap();
        for cap in p.captures_iter(&l) {
            if !rstructs.contains(&cap[1].trim().to_owned()) || in_block {
                let fields: Vec<String> = get_struct_fields(&cap[2].trim());
                let s = Struct::new(&cap[1].trim(), fields);
                structs.push(s.clone());
                rstructs.push(cap[1].trim().to_owned());
                continue;
            }
        }
        p = Regex::new("(.*)=.*\"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            if !rvars.contains(&cap[1].trim().to_owned()) || in_block {
                let v = Variable::new(&cap[1].trim(), &cap[2].trim());
                vars.push(v.clone());
                rvars.push(v.get_key());
                continue;
            }
        }
        p = Regex::new("(.*)=(.*).new(((.*)))").unwrap();
        for cap in p.captures_iter(&l) {
            for s in &mut structs {
                if s.get_name() == cap[2].trim().to_owned() {
                    s.set_variable(cap[1].trim());
                    s.set_values(get_struct_values(cap[3].trim()));
                    if !rvars.contains(&s.get_variable().to_owned()) {
                        let v = Variable::new(s.get_variable(), "[Struct]");
                        vars.push(v.clone());
                        rvars.push(v.get_key());
                    }
                }
            }
            continue;
        }
        p = Regex::new("task :.* do #[ignore]").unwrap();
        if p.is_match(&l) {
            continue;
        }
        p = Regex::new("task :(.*) do").unwrap();
        for cap in p.captures_iter(&l) {
            name = cap[1].to_owned();
            depends = String::new();
            continue;
        }
        p = Regex::new(&format!("task :(.*) => {}*:*(.*){} do",
        regex::escape("["), regex::escape("]"))).unwrap();
        if p.is_match(&l) {
            for cap in p.captures_iter(&l) {
                name = cap[1].to_owned();
                depends = cap[2].to_owned();
                continue;
            }
        } else {
            p = Regex::new(&format!("task :(.*) => {}*:*(.*){}",
            regex::escape("["), regex::escape("]"))).unwrap();
            for cap in p.captures_iter(&l) {
                name = cap[1].to_owned();
                depends = cap[2].to_owned();
                tasks.push(Task::new(&name, &depends, "", "", i));
                continue;
            }
        }
        p = Regex::new("(puts) \"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            tasks.push(Task::new(&name, &depends, &command, &params, i));
            l_puts = true;
            continue;
        }
        p = Regex::new("(puts)").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = String::new();
            if !l_puts {
                tasks.push(Task::new(&name, &depends, &command, &params, i));
            }
            continue;
        }
        p = Regex::new("(puts) (.*).to_h.to_json").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            params = format!("#{{{}}}", &params);
            tasks.push(Task::new(&name, &depends, &command, &params.trim(), i));
            l_puts = true;
            continue;
        }
        /*p = Regex::new(".write.*(.*)").unwrap();
        for cap in p.captures_iter(&l) {
            command = "File.write".to_owned();
            params = format!("{}", &cap[1]); 
            tasks.push(Task::new(&name, &depends, &command, &params.trim(), i));
            continue;
        }*/
        p = Regex::new("(sleep) (.*)").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            tasks.push(Task::new(&name, &depends, &command, &params.trim(), i));
        }
        p = Regex::new("(Dir.pwd)").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            tasks.push(Task::new(&name, &depends, &command, "", i));
        }
        p = Regex::new("(Dir.chdir).*\"(.*)\"").unwrap();
        if p.is_match(&l) {
            for cap in p.captures_iter(&l) {
                command = cap[1].to_owned();
                params = cap[2].to_owned();
                tasks.push(Task::new(&name, &depends, &command, &params, i));
                continue;
            }
        } else {
            p = Regex::new("(Dir.chdir)(((.*)))").unwrap();
            for cap in p.captures_iter(&l) {
                command = cap[1].to_owned();
                params = cap[2].to_owned();
                params = format!("#{{{}}}", &params[1..params.len() - 2]);
                tasks.push(Task::new(&name, &depends, &command, &params, i));
            }
        }
        p = Regex::new("(sh) \"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            tasks.push(Task::new(&name, &depends, &command, &params, i));
        }
        p = Regex::new("(ruby) \"(.*)\"").unwrap();
        for cap in p.captures_iter(&l) {
            command = "sh".to_owned();
            params = format!("ruby {}", cap[2].to_owned());
            tasks.push(Task::new(&name, &depends, &command, &params, i));
        }
        p = Regex::new("(File.delete).*\"(.*)\"").unwrap();
        if p.is_match(&l) {
            for cap in p.captures_iter(&l) {
                command = cap[1].to_owned();
                params = cap[2].to_owned();
                tasks.push(Task::new(&name, &depends, &command, &params, i));
                continue;
            }
        } else {
            p = Regex::new("(File.delete)(((.*)))").unwrap();
            for cap in p.captures_iter(&l) {
                command = cap[1].to_owned();
                params = cap[2].to_owned();
                params = format!("#{{{}}}", &params[1..params.len() - 2]);
                tasks.push(Task::new(&name, &depends, &command, &params, i));
            }
        }
        p = Regex::new(r"(FileUtils.copy)(((.*)))").unwrap();
        for cap in p.captures_iter(&l) {
            command = cap[1].to_owned();
            params = cap[2].to_owned();
            let ip = Regex::new("(.*), (.*)").unwrap();
            let mut fparams = String::new();
            for cap in ip.captures_iter(&params) {
                fparams = format!("#{{{}}} #{{{}}}", &cap[1][1..cap[1].len()], 
                &cap[2][0..cap[2].len() - 2]);
            }
            tasks.push(Task::new(&name, &depends, &command, &fparams, i));

        }
        p = Regex::new(&format!("if OS.windows{} then", regex::escape("?"))).unwrap();
        if p.is_match(&l) {
            if get_os() == "Unknown" {
                in_block = true;
            } else {
                in_block = false;
            }
        }
    }

    let pvars = process_struct_vars(&structs, process_vars(rvars, vars));
    //println!("Vars = {:#?}", pvars);
    let mut ptasks: Vec<Task> = Vec::new();
    for task in &tasks {
        let ptask = parse_vars_in_task(&task, &pvars);
        ptasks.push(ptask);
    }
    //println!("Tasks = {:#?}", ptasks);

    let mut matched = false;
    let mut qtask = String::new();
    let mut rtasks: Vec<Task> = Vec::new();
    let mut lnos: Vec<usize> = Vec::new();
    for stask in stasks {
        qtask = stask.to_owned();
        for task in ptasks.clone() {
            if task.get_name() == stask {
                matched = true;
                let depends = task.get_depends();
                if !depends.is_empty() {
                    for dtask in ptasks.clone() {
                        if dtask.get_name() == depends 
                        && !lnos.contains(&dtask.get_line()) {
                            rtasks.push(dtask.clone());
                            lnos.push(dtask.get_line());
                        }
                    }
                }
                rtasks.push(task.clone());
            }
        }
    }
    //println!("{:#?}", rtasks);
    if !matched {
        throw_no_task_failure(&program, &qtask);
    }
    let mut wkdir = env::current_dir().unwrap();
    for task in &rtasks {
        match task.get_command() {
            "puts" => if opts.verbose { println!("{}", task.get_params()) },
            "sleep" => { thread::sleep_ms(parse_unit(task.get_params()) as u32) },
            "sh" => {
                if opts.verbose {
                    println!("{}", task.get_params());
                }
                let split = task.get_params().split(" ");
                let mut args: Vec<&str> = split.collect();
                let cmd = args[0]; args.remove(0);
                let mut output = Command::new(&cmd)
                .args(&args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .current_dir(&format!("{}", wkdir.display()))
                .spawn()
                .unwrap();
                let status = output.wait();
                let strstat = format!("{:?}", status);
                let p = Regex::new("([0-9]+)").unwrap();
                let mut code = String::new();
                for cap in p.captures_iter(&strstat) {
                    code = cap[0].to_owned();
                }
                let ec = parse_unit(code.trim());
                if opts.exit_codes {
                    println!("Exited with code {}", ec);
                }
                if ec != 0 && !opts.ignore {
                    throw_build_failiure(&program, &qtask, ec, task.get_line());
                }
            },
            "Dir.pwd" => {
                println!("{}", wkdir.display());
            },
            "Dir.chdir" => {
                assert!(env::set_current_dir(&task.get_params()).is_ok());
                wkdir = env::current_dir().unwrap();
            },
            "File.delete" => {
                let file = &task.get_params();
                if Path::new(file).exists() {
                    fs::remove_file(file).unwrap()
                }
            },
            "FileUtils.copy" => {
                let sd = &task.get_params();
                let p = Regex::new("(.*) (.*)").unwrap();
                for cap in p.captures_iter(&sd) {
                    fs::copy(&cap[1], &cap[2]).unwrap();
                }
            },
            "File.write" => {
                let c = &task.get_params();
                let mut w = File::create("file").unwrap();
                let _ = w.write_all("foo".as_bytes());
            },
            _ => {},
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

fn throw_build_failiure(program: &str, task: &str, ec: i32, line: usize) {
    println!("{} aborted!", program);
    println!("Failed to build task '{}' at line {}", task, line);
    println!("Exited with code: {}", ec);
    exit(ec);
}

fn throw_bad_format_file(program: &str, file: &str) {
    println!("{} aborted!", program);
    println!("Provided file '{}' does not seem to be in Rakefile format.", file);
    exit(-1);
}

fn display_version() {
    println!("rake in rust, version 0.1.0");
    exit(0);
}

fn display_usage(program: &str, code: i32) {
    println!("Rake in Rust.");
    println!("Ruby build tool implementation.");
    println!("Copyright 2017 Sam Saint-Pettersen.");
    println!("\nReleased under the MIT License.");
    println!("\nUsage: {} [-f|--rakefile <rakefile>] [options] [task]", program);
    println!("\nOptions are:\n");
    println!("-q | --quiet: Do not print out to stdout other than sh stdout/stderr (Quiet mode).");
    println!("-e | --exits: Print exit codes for sh invokations.");
    println!("-i | --ignore-ec: Ignore bad exit codes and continue.");
    println!("-x | --ignore-ext: Ignore extension for Rakefile.");
    println!("-m | --ignore-format: Ignore format for Rakefile.");
    exit(code);
}

fn main() {
    let cli = CliOptions::new("rrake");
    let program = cli.get_program();
    // ------------------------------------------------------------------------
    let rakefiles = vec!["rakefile", "Rakefile", "rakefile.rb", "Rakefile.rb"];
    // ------------------------------------------------------------------------
    let mut tasks: Vec<String> = Vec::new();
    let mut srakefile = String::new();
    let mut verbose = true;
    let mut exit_codes = false;
    let mut ignore = false;
    let mut ext = true;
    let mut format = true;

    if cli.get_num() > 1 {
        for (i, a) in cli.get_args().iter().enumerate() {
            match a.trim() {
                "-h" | "--help" => display_usage(&program, 0),
                "-v" | "--version" => display_version(),
                "-q" | "--quiet" => verbose = false,
                "-e" | "--exits" => exit_codes = true,
                "-f" | "--rakefile" | "--file" => srakefile = cli.next_argument(i),
                "-i" | "--ignore-ec" => ignore = true,
                "-x" | "--ignore-ext" => ext = false,
                "-m" | "--ignore-format" => format = false,
                _ => tasks.push(a.to_owned()),
            }
        }
    }

    let opts = Options { 
        verbose: verbose, 
        exit_codes: exit_codes,
        ignore: ignore,
    };

    let mut tasks = parse_tasks(&program, tasks);
    if tasks.len() == 0 {
        tasks.push("default".to_owned());
    }
    if !srakefile.is_empty() {
        if (ext && !validate_extension(&srakefile)) 
        || (format && !validate_rakefile(&srakefile)) {
            throw_bad_format_file(&program, &srakefile);
        }
        if Path::new(&srakefile).exists() {
            invoke_rakefile(&program, &srakefile, &tasks, &opts);
        }
    }

    for rakefile in &rakefiles {
        if (ext && !validate_extension(&rakefile))
        || (format && !validate_rakefile(&rakefile)) {
            throw_bad_format_file(&program, &rakefile);
        }
        if Path::new(&rakefile).exists() {
            invoke_rakefile(&program, &rakefile, &tasks, &opts);
        }
    }
    throw_not_found_failure(&program, &rakefiles);
}
