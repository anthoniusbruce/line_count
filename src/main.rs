use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "line_count",
    about = "counts the new-line characters in a file. \tline_count <file_name>"
)]
struct Opt {
    /// input file
    #[structopt(parse(from_os_str), required(true))]
    file_name: PathBuf,
}

fn main() {
    let opt = Opt::from_args_safe();
    match opt {
        Ok(args) => count_lines(args.file_name),
        Err(e) => println!("{e}"),
    };
}

fn count_lines(file_name: PathBuf) {
    let file_contents = read_file(file_name);
    match file_contents {
        Ok(contents) => report_line_count(contents),
        Err(e) => println!("{e}"),
    };
}

fn report_line_count(contents: String) {
    let lines = contents.split('\n');
    println!("{}", lines.count());
}

fn read_file(file_name: PathBuf) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file_name)?;
    Ok(contents)
}
