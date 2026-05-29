use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;

fn main() -> Result<(), reqwest::Error> {
    //Generate a reusable http client
    let tear_client = Client::new();
    //Create a response string for active alerts to populate to
    let response = tear_client.get("https://api.weather.gov/alerts/active")
        //USER_AGENT is needed by the API to identify the application
        .header(USER_AGENT, "TEAR/1.0")
        //Query the API | ? unwraps the success or returns the error
        .send()?
        //Grab the text from the API
        .text()?;
    //Display GeoJSON
    println!("Response = {}", response);
    Ok(())
}