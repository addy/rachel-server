use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub subject: String,
    pub text: String,
    pub html: String,
}

impl Message {
    pub fn new(first_name: String, last_name: String, email: String, message: String) -> Self {
        let subject = format!("rachelshawstudio.com - Contact from {last_name}, {first_name} <{email}>");
        let text = format!("Rachel Shaw Studio - Contact Form\nMessage:\n{message}");
        let html = format!("<h1>Rachel Shaw Studio - Contact Form</h1><p>{message}</p><a href=\"rachelshawstudio.com\">rachelshawstudio.com</a>");

        Self {
            subject: subject.to_string(),
            text: text.to_string(),
            html: html.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContactResponse {
    pub success: bool,
}
