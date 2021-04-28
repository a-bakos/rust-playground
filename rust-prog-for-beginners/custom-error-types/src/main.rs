use thiserror::Error;

#[derive(Error, Debug)]
enum LoginError {
    #[error("database error")]
    DatabaseError(#[from] SqlError),

    #[error("pass expired")]
    PasswordExpired,

    #[error("user not found")]
    UserNotFound,

    #[error("network connection error")]
    NetworkError(#[from] std::io::Error),

    #[error("wrong pass")]
    WrongPassword,
}

fn login(name: &str, password: &str) -> Result<String, LoginError> {
    let connection: Result<Connection, std::io::Error> = connect()?;
    let user_id = get_user_id(name)?;
    if try_passsword(user_id, password)? {
        let session: Result<String, SqlError> = get_session(user_id)?;
        Ok(session)
    } else {
        Err(LoginError::WrongPassword)
    }
}

fn main() {
    login("kate", "123");
}
