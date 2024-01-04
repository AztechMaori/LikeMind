
mod db;
mod routes;
mod utils;
use routes::create_routes;
use sea_orm::{Database, DatabaseConnection, DbErr};

async fn validate_connection(db:Result<DatabaseConnection,DbErr>){
    match db {
        Ok(database) => {
            println!("Connection to database succesful!");
            let app =  create_routes(database);
            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, app).await.unwrap();

        } 
        Err(error)=> {
            println!("Unsuccessful connection to the database: {}", error);
        }
    }
}


pub async fn run(db_uri: String){
// connect SEAORM and DB 
let database: Result<DatabaseConnection,DbErr> = Database::connect(db_uri).await;

validate_connection(database).await;




// run server 
}





