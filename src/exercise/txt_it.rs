use std::collections::HashMap;

fn cmd_add_employee(employee_name: &str, department_name: &str, company: &mut HashMap<String, Vec<String>>) {
    company.entry(department_name.to_owned()).
    and_modify(|names_in_dep| names_in_dep.push(employee_name.to_owned())).
    or_insert(vec![employee_name.to_owned()]);

    println!("{:?}", company);
}

pub fn txt_it_cmd(cmd: &str, company: &mut HashMap<String, Vec<String>>) -> Result<&'static str, &'static str> {
    match cmd.strip_prefix("Add") {
        None => Err("cmd err"),
        Some(text) => {
            match text.find("to") {
                Some(index) => {
                    let (employee, department) = text.split_at(index);
                    cmd_add_employee(employee.trim(), department.strip_prefix("to ").unwrap(), company);
                    Ok("cmd executed")
                }
                None => Err("cmd err")
            }
        }
    }
}

