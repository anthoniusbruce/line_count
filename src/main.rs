use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "line_count",
    about = "counts the new-line characters in a file. Optionally, you can pass a word or character to count instead."
)]
struct Opt {
    /// input file
    #[structopt(parse(from_os_str), required(true))]
    file_name: PathBuf,
    #[structopt(default_value = "\n")]
    count_word: String,
}

fn main() {
    let opt = Opt::from_args_safe();
    match opt {
        Ok(args) => count_lines(args.file_name, args.count_word),
        Err(e) => println!("{e}"),
    };
}

fn count_lines(file_name: PathBuf, count_word: String) {
    let file_contents = read_file(file_name);
    match file_contents {
        Ok(contents) => report_line_count(contents, count_word),
        Err(e) => println!("{e}"),
    };
}

fn report_line_count(contents: String, count_word: String) {
    let lines = contents.split(&count_word);
    println!("{}", lines.count());
}

fn read_file(file_name: PathBuf) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file_name)?;
    Ok(contents)
}
