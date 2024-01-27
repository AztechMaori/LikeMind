
use backend::run;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // build our application with a single route
    dotenv().ok();
    // let db_uri = dotenv!("DATABASE_URL");
    let db_uri = std::env::var("DATABASE_URL").expect("db url could not be loaded from env");
    run(db_uri).await;
}
