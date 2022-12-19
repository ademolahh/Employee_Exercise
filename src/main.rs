use std::{collections::HashMap, io};
fn main() {
    let mut emp_list = HashMap::new();

    loop {
        println!("[0]: Enter employee name and department");
        println!("[1]: People in a department");
        println!("[2]: Exit");
        let mut enter_set = String::new();

        io::stdin().read_line(&mut enter_set).expect("error");

        let enter_set = enter_set.trim();

        match enter_set {
            "0" => {
                println!("Add employee name to department");

                let mut emp = String::new();
                io::stdin().read_line(&mut emp).expect("error");

                let emp = emp.trim();
                let emp_count = emp.split_ascii_whitespace().count();

                if emp_count > 3 {
                    let emp: Vec<&str> = emp.split_ascii_whitespace().collect();
                    let emp_name = emp.get(1).unwrap().to_string();
                    let emp_department = emp.get(3).unwrap().to_string();

                    emp_list.insert(emp_name, emp_department);
                } else {
                    println!("Enter in this format 'Add <name> to <department>'")
                }
            }

            "1" => {
                println!("People in department");

                let mut list = Vec::new();

                for (k, v) in &emp_list {
                    list.push(format!("{k}, {v}"));
                }

                list.sort();

                println!("List in dept {:?}", list);
                break;
            }
            "2" => {
                println!("Exited program");
                break;
            }

            _ => {
                println!("Input a valid entry")
            }
        }
    }
}
