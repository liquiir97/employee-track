
use axum::
{
    body::Body, http::{Request, Response, StatusCode}, middleware::{self, AddExtension, Next}, response::Json, routing::{get, patch, post}, Router   
};
use serde::{Deserialize, Serialize};

mod request;
mod dto;


use request::
{
    controller::employeeController, 
    employeeController::{add_employee_to_file_post, get_not_onboarded, generate_pass_and_handler, secured_han},
    service::employeeService
};

#[tokio::main]
async fn main() {
    
    let app = Router::new()

        .route("/employee", post(add_employee_to_file_post))
        .route("/notonboarded", get(get_not_onboarded))
        .route("/onboarding/:id", patch(generate_pass_and_handler))
        .route("/details", get(secured_han));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
