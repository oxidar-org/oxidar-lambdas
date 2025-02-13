use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: u64,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged, rename = "snake_case")]
pub enum Role {
    Admin,
    SuperUser,
    User,
}
