use crate::logging::log::{log_error, log_info};

#[derive(Debug)]
pub struct XMLError;

impl std::fmt::Display for XMLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There was an error with the xml sending event")
    }
}

impl std::error::Error for XMLError {}

pub async fn send_xml_request(xml_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    match client
        .post("http://127.0.0.1:1234/print_xml")
        .header(reqwest::header::CONTENT_TYPE, "application/xml")
        .body(xml_content.to_string())
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                log_info("Sucessfully send XML request");
                Ok(())
            } else {
                log_error(&format!(
                    "Failed to send XML request: {}",
                    response.status()
                ));
                Err(Box::new(XMLError))
            }
        }
        Err(e) => {
            log_error(&format!(
                "There was an error in sending post request: {}",
                e
            ));
            Err(Box::new(XMLError))
        }
    }
}
