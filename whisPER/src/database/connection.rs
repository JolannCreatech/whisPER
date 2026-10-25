use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn connect() -> Result<(), sqlx::Error> {

    let user_key : &str = "RUST_APP_USR";
    let mdp_key : &str = "RUST_APP_PWD";
    let db_key : &str = "DB";

    let user = env::var(user_key).unwrap();
    let password = env::var(mdp_key).unwrap();
    let db = env::var(db_key).unwrap();

    let url : String = format!("postgres://{}:{}@localhost/{}", &user, &password, &db);
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    println!("Connected!");
    Ok(())
}