extern crate chrono;
use chrono::{DateTime,Utc};

use std::{
    fs::{File,OpenOptions},
    io::{prelude::*, BufReader,Write},
    path::Path,
};


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("employee data file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn print_diff(date_string: &str) -> String {
    let today = Utc::now();

    let datetime =
        DateTime::<Utc>::from_utc(chrono::NaiveDate::parse_from_str(date_string, "%d-%m-%Y")
            .unwrap()
            .and_hms(0, 0, 0), Utc);

    let diff = today.signed_duration_since(datetime);
    let days = diff.num_days();
    let s: String = days.to_string();
    s
} 

pub fn run(){
    

    //Emp Id and Name from Employee Data File 
    let lines = lines_from_file("employee data file.txt");
    let emp1: Vec<&str> = lines[1].split('|').collect();
    let emp2: Vec<&str> = lines[2].split('|').collect();
    let emp_id1 = emp1[0];
    let emp_id2 = emp2[0];
    let mut emp1_output = String::from(emp_id1);
    let mut emp2_output = String::from(emp_id2);
    let emp_name1 = emp1[1];
    let emp_name2 = emp2[1];
    emp1_output.push('#');
    emp1_output.push_str(emp_name1);
    emp2_output.push('#');
    emp2_output.push_str(emp_name2);
    
   
    //Dept Title from Department Data File
    let lines = lines_from_file("department data file.xls");
    let emp1: Vec<&str> = lines[1].split('|').collect();
    let emp2: Vec<&str> = lines[2].split('|').collect();
    let emp_dept1 = emp1[1];
    let emp_dept2 = emp2[1];
    emp1_output.push('#');
    emp1_output.push_str(emp_dept1);
    emp2_output.push('#');
    emp2_output.push_str(emp_dept2);

    //Mobile No and email from Employee Data File
    let lines = lines_from_file("employee data file.txt");
    let emp1: Vec<&str> = lines[1].split('|').collect();
    let emp2: Vec<&str> = lines[2].split('|').collect();
    let emp_mobile1 = emp1[3];
    let emp_mobile2 = emp2[3];
    emp1_output.push('#');
    emp1_output.push_str(emp_mobile1);
    emp2_output.push('#');
    emp2_output.push_str(emp_mobile2);
    let emp_email1 = emp1[4];
    let emp_email2 = emp2[4];
    emp1_output.push('#');
    emp1_output.push_str(emp_email1);
    emp2_output.push('#');
    emp2_output.push_str(emp_email2);

    //Salary status from salary data file
    let lines = lines_from_file("salary data file.xlsx");
    let emp1: Vec<&str> = lines[1].split('|').collect();
    let emp2: Vec<&str> = lines[2].split('|').collect();
    let emp_salary1 = emp1[4];
    let emp_salary2 = emp2[4];
    emp1_output.push('#');
    emp1_output.push_str(emp_salary1);
    emp2_output.push('#');
    emp2_output.push_str(emp_salary2);

    //Leave Days from leave Data File
    let lines = lines_from_file("leave data file.xlsx");
    let emp1: Vec<&str> = lines[1].split('|').collect();
    let emp2: Vec<&str> = lines[2].split('|').collect();
    let emp_leave_from_1 = emp1[2];
    let emp_leave_from_2 = emp2[2]; 
    emp1_output.push('#');
    if emp_leave_from_1.is_empty(){
        emp1_output.push_str("0");
    } else{
        emp1_output.push_str(&*print_diff(emp_leave_from_1));
    }
    emp2_output.push('#');
    if emp_leave_from_2.is_empty(){
        emp2_output.push_str("0");
    } else{
        emp2_output.push_str(&*print_diff(emp_leave_from_2));
    }
    
    //Append data in Output Data File 
    
    let mut file = OpenOptions::new().append(true).open("output data file.txt").expect(
            "cannot open output data file");
    file.write_all(emp1_output.as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed"); 
    file.write_all(emp2_output.as_bytes()).expect("write failed");
        

    //Output Data File
    let mut lines = lines_from_file("output data file.txt");
    for line in lines {
        println!("{:?}", line);
    } 
}
