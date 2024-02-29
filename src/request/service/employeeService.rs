use std::{fs::File, io::{Read, Write}};

use crate::dto::employee::{self, Employee};
use axum::{http::StatusCode, response::IntoResponse};
use axum::response::Json;
use rand::seq::SliceRandom;

use serde::{Deserialize, Serialize};

pub fn read_employees_from_file(file_name: &str) -> Vec<Employee>
{

    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            return Vec::new();
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_default();

    serde_json::from_str(&contents).unwrap_or_default()
}

pub fn write_employees_to_file(employees: &[Employee], file_name: &str) {
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to create file.");
            return;
        }
    };

    let json_data = serde_json::to_string(&employees).unwrap_or_default();
    if let Err(_) = file.write_all(json_data.as_bytes()) {
        eprintln!("Failed to write to file.");
    }
}

pub fn add_employee_to_file(employee: Employee, file_name: &str){
    let mut employees = read_employees_from_file(file_name);
    employees.push(employee);
    write_employees_to_file(&employees, file_name);
}

pub fn get_last_id() -> i64
{
    let file_name = "employees.json";
    let employees = read_employees_from_file(file_name);
    
    let employee = employees.last();
    match employee {
        Some(employee) => employee.id+1,
        None => 1,
    }
}

pub fn get_user_not_onboarded() -> Vec<i64>
{
    let file_name = "employees.json";
    let employees = read_employees_from_file(file_name);
    let mut idS : Vec<i64> = vec![];
    if !employees.is_empty()
    {
        let employee : Vec<Employee> = employees.into_iter().filter(|e| e.password.is_none() && e.global_handler.is_none()).collect();
        for emplo in employee.iter()
        {
            idS.push(emplo.id);
        }
        
    }
    return idS;
}


pub fn generate_pass_and_handler_service(id : i64) -> String
{
    let file_name = "employees.json";
    let mut employees = read_employees_from_file(file_name);
    let mut employees_result = vec![];
    
    let mut found = false;
    for mut e in employees
    {
        if e.id == id //&& e.global_handler.is_none() && e.password.is_none()
        {
            e.global_handler = Some(e.first_name.to_string() + &e.last_name.to_string() + &e.id.to_string());
            e.password = Some(generate_password());
            found = true;
            //break;
        }
        employees_result.push(e);
    }

    if found
    {
        write_employees_to_file(&employees_result, file_name);

        return format!("Employee with id {} is updated.", id);
    }
    else
    {
        return format!("There is no employee with id: {}", id)
    }
    
}


fn generate_password() -> String
{
    //generate letters
    let mut letters = ['a'; 26];
    for (i, letter) in letters.iter_mut().enumerate() {
        *letter = (b'a' + i as u8) as char;
    }

    //mix them
    let mut rng = rand::thread_rng();
    letters.shuffle(&mut rng);

    //take 6
    let mut selected_elements: Vec<char> = letters.iter().take(6).cloned().collect();
    selected_elements[1] = selected_elements[1].to_ascii_uppercase();

    let first_character = selected_elements[0];

    selected_elements.remove(0);

    let mut special_character = ["!", "?", ".", ",", "#"];
    special_character.shuffle(&mut rng);

    let selected_character = special_character[0];

    let mut numbers = ["0","1","2","3","4","5","6","7","8","9"];
    numbers.shuffle(&mut rng);


    let mut selected_numbers : Vec<&str> = numbers.iter().take(2).cloned().collect();

    let mut initial_pass = "".to_string();

    for l in selected_elements
    {
        initial_pass += &l.to_string();
    }
    
    initial_pass += selected_character;

    let last_character = selected_numbers[1];

    selected_numbers.remove(1);
    for n in selected_numbers
    {
        initial_pass += &n.to_string();
    }

    let sliced_password = slice_password(initial_pass);

    let final_password = first_character.to_string() + &sliced_password + last_character;
    let final_password_for_true = final_password.clone();
    let mut final_password_clone: Option<String> = Some(final_password);
    
    let file_name = "employees.json";
    let employees = read_employees_from_file(file_name);

    let is_employee : Vec<Employee> = employees.into_iter().filter(|e| e.password.eq(&final_password_clone)).collect();

    //recursive call if there is already password so to get unique
    if is_employee.is_empty()
    {
        return final_password_for_true;

    }
    else
    {
        return generate_password();
    }
    
}

fn slice_password(password : String) -> String
{
    let mut rng = rand::thread_rng();
    let mut bytes = password.into_bytes();
    bytes.shuffle(&mut rng);
    
    let mut sliced_password = String::from_utf8(bytes).unwrap();

    return sliced_password;
}

pub fn get_empoyee_data(password : Option<String>) -> Result<Json<Employee>, StatusCode>
{
    let file_name = "employees.json"; 
    let mut employees = read_employees_from_file(file_name);

    let employee = employees.into_iter().find(|e| e.password == password);

    if let Some(employee_exist) = employee
    {
        return Ok(Json(employee_exist));
    }
    else
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
}







