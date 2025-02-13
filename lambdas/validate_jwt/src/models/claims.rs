use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
}

#[derive(PartialEq, Deserialize, Debug)]
#[serde(untagged, rename = "snake_case")]
pub enum Role {
    Admin,
    SuperUser,
    User,
}
