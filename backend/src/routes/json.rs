use axum:: extract::Json; 
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User{
    name: String, 
    age:i32,
}

#[derive(Serialize)]
pub struct Response {
    name: String, 
    age:i32, 
    stuff:i32,
}

pub async fn return_json(Json(payload):Json<User>) -> Json<Response>{
  return Json(Response{name:payload.name, age:payload.age, stuff:69});
}

#[derive(Deserialize)]
pub struct Car{
    name:String, 
    age:i32, 
    price:i32, 
}

#[derive(Serialize)]
pub struct Car_Response {
    name:String,
    sold:bool, 
    price_in_ten_years: i32, 
}

pub async fn car_details(Json(car):Json<Car>) -> Json<Car_Response>{

    let new_price = car.price - 2000;




    return Json(Car_Response{ name:car.name,price_in_ten_years: new_price, sold:true});
}