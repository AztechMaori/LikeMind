use argon2::{password_hash::SaltString, Argon2, PasswordHasher};

pub fn validate_password(user_pw:String, db_password:String, salt_string:String) -> bool{
    let argon = Argon2::default();
    let salt = SaltString::from_b64(&salt_string).expect("error converting string");
    let hashed_pw = argon.hash_password(user_pw.as_bytes(),&salt).expect("password hashing failed").to_string();  

    if hashed_pw == db_password {
        return true 
    }
    else {
        return false
    }
}