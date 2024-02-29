use axum::
{
    body::{Body, HttpBody}, extract::{path, Path}, http::{response, StatusCode}, response::{IntoResponse, Json, Response}, routing::{get, patch, post}, Error, Router    
};

use rand::seq::SliceRandom;
use axum_auth::AuthBasic;

use serde::Deserialize;
use crate::dto::employee::{self, Employee};

use std::{f32::consts::E, fs::File, io::{Read, Write}};

use employeeService::write_employees_to_file;
use employeeService::read_employees_from_file;
use employeeService::add_employee_to_file;
use employeeService::get_last_id;
use employeeService::get_user_not_onboarded;
use employeeService::generate_pass_and_handler_service;
use employeeService::get_empoyee_data;
#[path ="../service/employeeService.rs"]
pub mod employeeService;

#[derive(Deserialize)]
pub struct NewEmployee {
    first_name: String,
    last_name: String,
    age: u8,
    telephone: String,
    diploma: Option<Vec<String>>,
    password: Option<String>,
    global_handler: Option<String>
}
#[derive(Deserialize)]
pub struct EmployeeToUpdate
{
    id : i64
}

pub async fn add_employee_to_file_post(Json(new_employee): Json<NewEmployee>) -> Response<Body>//Json<&'static str>
{
    
    if new_employee.age < 18
    {
        return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Employee must be at least 18 years old or more to be added!"))
        .unwrap();
        //return Json("Employee must be at least 18 years old or more to be added!")
    }    
    if new_employee.diploma.is_none()
    {
        return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Employee must have at least one diploma to be added!"))
        .unwrap();
        //return Json("Employee must have at least one diploma to be added!");
    }

    let id_new = get_last_id();
    let employee = Employee {
        id: id_new,
        first_name: new_employee.first_name,
        last_name: new_employee.last_name,
        age: new_employee.age,
        telephone: new_employee.telephone,
        diploma: new_employee.diploma,
        password: new_employee.password,
        global_handler: new_employee.global_handler,           
        
    };
    let file_name = "employees.json";
    add_employee_to_file(employee, file_name);
    //return Json("Employee is added!")
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Employee is added!"))
        .unwrap()
}

pub async fn get_not_onboarded() -> Json<Vec<i64>>
{
    let ids = get_user_not_onboarded();
    return Json(ids);
}


pub async fn generate_pass_and_handler(Path(id): Path<i64>) -> Json<String>
{
    let response = generate_pass_and_handler_service(id);
    return Json(response);
}


pub async fn secured_han(AuthBasic((id, password)): AuthBasic) -> Result<Json<Employee>, StatusCode>
{
    let is_white_space = match &password {
        Some(password_data) => password_data.trim().is_empty(),
        None => false,
    };

    if is_white_space
    {
        return Err(StatusCode::UNAUTHORIZED);
    }
    else
    {
        return get_empoyee_data(password);
    }
}

