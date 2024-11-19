use std::result::Result;
use std::fs::File;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde::{Serializer, Deserializer};
use serde_json::{self};
use url::Url;
use uuid::Uuid;
use toml;
use serde_yaml::to_string as to_yaml;
use toml::to_string as to_toml;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration, // Duration
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url, //url
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug{
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>, //data
}

#[derive(Debug, Serialize, Deserialize)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let user = User{
    //     name:"John".to_string(),
    //     email:"johndoe321@gmail.com".to_string(),
    //     birthdate:"5.06.97".to_string()
    // };
    //
    // let json = serde_json::to_string(&user)?;
    // println!("{}", json);
    //
    // let deser_user: User = serde_json::from_str(&json)?;
    // println!("{:?}", deser_user);

    let event: Event = Event {
        name: "Event 1".to_string(),
        date: "2021-06-01".to_string(),
    };
    let json: String = serde_json::to_string(&event).unwrap();
    println!("{}", json);

    let des_event: Event = serde_json::from_str(&json).unwrap();
    println!("{:?}", des_event);

    let file = File::open("request.json")?;
    let request: Request = serde_json::from_reader(file)?;

    println!("{:#?}", request);

    println!("yaml: {}", to_yaml(&request)?);
    println!("toml: {}", to_toml(&request)?);

    Ok(())
}


#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(serialize_with = "serialize_date", deserialize_with = "deserialize_date")]
    date: String,
}


fn serialize_date<S: Serializer>(date: &str, serializer: S) -> std::result::Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'de, D: Deserializer<'de>>(deserializer: D) -> std::result::Result<String, D::Error> {
    let data: String = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date: ", ""))
}






#[cfg(test)]
mod  tests {
    use std::{fs::read_to_string, io::Read};
    use super::*;

    #[test] 
    fn test_1() {
        let mut file: File = File::open("request.json").unwrap();
        let mut serde_json = String::new();
        file.read_to_string(&mut serde_json).unwrap();
        let content = read_to_string("request.json").unwrap();
        let request: Request = serde_json::from_str(&content).unwrap();
        assert_eq!(request.stream.private_tariff.client_price, 250);
        assert_eq!(request.stream.user_id, Uuid::parse_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap());
        assert_eq!(request.gifts[0].id, 1);
        assert_eq!(request.gifts[0].price, 2);
        assert_eq!(request.gifts[0].description, "Gift 1");
    }

}