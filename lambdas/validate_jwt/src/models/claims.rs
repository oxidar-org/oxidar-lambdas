use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Claims {
    sub: String,
    roles: Vec<Role>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin,
    SuperUser,
    User,
}
