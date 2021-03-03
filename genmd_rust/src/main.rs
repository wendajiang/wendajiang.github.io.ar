use std::{env, fmt, io::Write, process::exit};
// use std::fs;
use std::time::{SystemTime};
use chrono::offset::Utc;
use chrono::DateTime;
use std::fs::OpenOptions;
use chrono::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // println!("{}", args.len());
    if args.len() < 2 {
        println!("Usage: genmd file_name [-p path]");
        exit(0);
    }

    let mut file_path = "./_posts/";
    if args.len() == 4 {
        file_path = &args[3];
        println!("-p file_path is {}", file_path);
    }

    let pre_content = "---\nlayout: posts\ndate: ";

    let utc_time: DateTime<Utc> = Utc::now();       
    let local_time: DateTime<Local> = Local::now(); 

    // println!("{}", utc_time.format("%Y-%m-%d %T"));
    // println!("{}", local_time.format("%Y-%m-%d %T"));
    let str_datetime = local_time.format("%Y-%m-%d %T");
    let file_prefix = local_time.format("%Y-%m-%d");

    let file_name = std::format!("{}-{}.md", file_prefix, &args[1]);
    println!("file_name is {}", file_name);

    let content = std::format!("{}{}\ncategories:\ntitle:\n---\n", pre_content, str_datetime);
    println!("{}", content);

    let full_file_name = std::format!("{}{}", file_path, file_name);

    let mut file = std::fs::OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(full_file_name).unwrap();
    
    file.write_fmt(format_args!("{}", content)).unwrap();

}
