use std::{io::Write};
use chrono::DateTime;
use chrono::prelude::*;
use clap::{Arg, App};


fn main() {
    let matches = App::new("genmd")
        .version("1.0")
        .author("david")
        .about("generate md file")
        .arg(Arg::new("file_path")
            .short('p')
            .long("file_path")
            .value_name("FILE_PATH")
            .about("get the file path, example ./help")
            .takes_value(true)
            )
        .arg(Arg::new("title")
            .short('t')
            .long("title")
            .value_name("TITLE")
            .takes_value(true)
            .about("get the blog title")
            )
        .arg(Arg::new("mermaid")
            .short('m')
            .long("mermaid")
            .value_name("mermaid true/false")
            .about("-m/--mermaid true to turn on mermaid")
            )
        .arg(Arg::new("file_name")
            .required(true)
            .index(1))
        .get_matches();

    let file_path = matches.value_of("file_path").unwrap_or("./content/");
    let title = matches.value_of("title").unwrap_or("");
    let file_name_in = matches.value_of("file_name").unwrap();
    let mermaid_flag = matches.value_of("mermaid").unwrap_or("false");
    let mut mermaid_template = "<!--\nmermaid example:\n<div class=\"mermaid\">\n    mermaid program\n</div>\n-->";
    if mermaid_flag == "false" {
        mermaid_template = "";
    }


    let pre_content = "+++\ntemplate = \"page.html\"\n";

    // let utc_time: DateTime<Utc> = Utc::now();
    let local_time: DateTime<Local> = Local::now();

    // println!("{}", utc_time.format("%Y-%m-%d %T"));
    // println!("{}", local_time.format("%Y-%m-%d %T"));
    let str_datetime = local_time.format("%Y-%m-%d %T");

    let file_name = std::format!("{}{}.md", file_path, file_name_in);
    println!("file_name is {}", file_name);

    let content = std::format!("{}date = \"{}\"\ntitle = \"{}\"\n[taxonomies]\ntags = []\n\n[extra]\nmermaid = {}\nusemathjax = true\n+++\n{}", pre_content, str_datetime, title, mermaid_flag, mermaid_template);
    println!("{}", content);

    let mut file = std::fs::OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(file_name).unwrap();

    file.write_fmt(format_args!("{}", content)).unwrap();

}
