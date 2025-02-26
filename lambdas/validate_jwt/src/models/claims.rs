use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<Role>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    SuperUser,
    User,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Admin => "admin",
            Role::SuperUser => "super_user",
            Role::User => "user",
        }
    }
}
