extern crate clap;
use clap::{Arg, App};

#[derive(Debug)]
pub struct Arguments {
    pub emp_data_file_path: String,
    pub dept_data_file_path: String,
    pub salary_data_file_path: String,
    pub leave_data_file_path: String,
    pub output_data_file_path: String,
    pub dept_data_sheet_name: String,
    pub salary_data_sheet_name: String,
    pub leave_data_sheet_name: String
}

pub fn get_arguments() -> Arguments{
    let matches = App::new("assignment")
    .version("0.1.0")
    .author("Harsh Kushwah <harsh.sk@surya-soft.com>")
    .about("Implementation of assignment-1")
    .arg(Arg::new("e")
             .short('e')
             .required(true)
             .long("emp-data-file-path")
             .takes_value(true)
             .help("A file having details about employee"))
    .arg(Arg::new("d")
             .short('d')
             .required(true)
             .long("dept-data-file-path")
             .takes_value(true)
             .help("A file having details about deparments"))
    .arg(Arg::new("s")
             .short('s')
             .required(true)
             .long("salary-data-file-path")
             .takes_value(true)
             .help("A file having details about salary"))
    .arg(Arg::new("l")
             .short('l')
             .required(true)
             .long("leave-data-file-path")
             .takes_value(true)
             .help("A file having details about leave"))
    .arg(Arg::new("o")
             .short('o')
             .required(true)
             .long("output-file-path")
             .takes_value(true)
             .help("A file having the outputs"))
    .arg(Arg::new("ds")
             .long("department-sheet-name")
             .takes_value(true)
             .default_value("Sheet1")
             .help("name of sheet in the file having the department details"))
    .arg(Arg::new("ss")
             .long("salary-sheet-name")
             .takes_value(true)
             .default_value("Sheet1")
             .help("name of sheet in the file having the salary details"))
    .arg(Arg::new("ls")
             .long("leave-sheet-name")
             .takes_value(true)
             .default_value("Sheet1")
             .help("name of sheet in the file having the leave details"))
    .get_matches();
}