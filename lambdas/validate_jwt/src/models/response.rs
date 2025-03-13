use serde_json::json;

pub enum Effect {
    Allow,
    Deny,
}

impl Effect {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allow => "Allow",
            Self::Deny => "Deny",
        }
    }
}

pub struct Response;

impl Response {
    pub fn with_params(effect: Effect, resource: String, principal_id: String) -> String {
        /*
        json!({
              "principalId": principal_id,
              "policyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                  {
                    "Action": "execute-api:Invoke",
                    "Effect": effect.as_str(),
                    "Resource": resource
                  }
                ]
              }
            }
        )
        .to_string()
        */
        json!({
              "principalId": "user",
              "policyDocument": {
                "Version": "2012-10-17",
                "Statement": [
                  {
                    "Action": "execute-api:Invoke",
                    "Effect": effect.as_str(),
                    "Resource": "*",
                  }
                ]
              }
            }
        )
        .to_string()
    }
}
