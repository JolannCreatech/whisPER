// We want to be able to :
//  - create a user
//  - add the new user to the database
//  - get a user from the data base
//  - verify the user
//  - delete a User

use crate::crypto;
use use sqlx::postgres::;

struct User {
    id : i32,
    user_name : String,
    hash_password : String,
}

struct NewUser {
    user_name : String,
    hash_password : String,
}

impl NewUser {
    pub fn create_user(user_name: String, password: String) -> User {
        User {
            user_name,
            password: crypto::hashing::hash_password(&password),
        }
    }
    pub fn add_user_to_db(user: User) -> None {

    }
}

impl User {
    fn get_user_from_db_with_username (user_name: String) -> User{

    }

    fn get_user_from_db_with_id (id : i8) -> User

    pub fn verify_user(user_name: String, password: String) -> bool {
        crypto::hashing::verifiy_password(&password, get_user_from_db_with_username(user_name))
    }

}
