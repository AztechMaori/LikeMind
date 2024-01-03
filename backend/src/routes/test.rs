use axum::Extension;
use uuid::Uuid;



pub async fn trial(Extension(id): Extension<Uuid> ){
    println!("this works");
    println!("this is teh middleware working woo!:{}",id );
}