use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};

fn default_string() -> String {
    String::new()
}
fn default_number() -> String {
    "0.0".to_string()
}
fn default_date() -> DateTime<Utc> {
    Utc::now()
}
fn order_deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = match s.parse::<f32>() {
        Ok(f) => f.to_string(),
        Err(_) => "0.0".to_string(),
    };
    Ok(s)
}
fn date_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let date = match chrono::DateTime::parse_from_rfc3339(&s) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => Utc::now(),
    };
    Ok(date)
}
fn date_serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format("%Y-%m-%dT%H:%M:%S"));
    serializer.serialize_str(&s)
}
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub email: String,
    pub name: String,
    #[serde(rename = "clientRefId")]
    pub client_ref_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserResponse {
    pub data: Option<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserEvent {
    // user id
    #[serde(rename = "id")]
    pub client_ref_id: String, // user id
    pub id_type: String,
    #[serde(default = "default_string")]
    pub region: String,
    #[serde(default = "default_string")]
    pub device_type: String,
    #[serde(default = "default_string")]
    pub country: String,
    #[serde(default = "default_date")]
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub created_at: DateTime<Utc>,
    pub payload: Payload,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub activity_type: String,
    #[serde(default = "default_string")]
    pub url: String,
    #[serde(default = "default_number")]
    #[serde(deserialize_with = "order_deserialize")]
    pub order_total: String,
    #[serde(default = "default_string")]
    pub order_id: String,
    #[serde(default = "default_string")]
    pub element_id: String,
}

impl UserEvent {
    pub fn as_insert_query(&self, user_id: u32) -> String {
        let uuid = uuid::Uuid::new_v4();
        format!(
            "INSERT INTO user_segment_analytics.events VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}','{}', '{}','{}')",
            uuid, //id VARCHAR,
            user_id, // user_id INT 32,
            self.client_ref_id, // client_ref_id VARCHAR,
            self.id_type, // id_type VARCHAR,
            self.region, // region VARCHAR,
            self.device_type, // device_type VARCHAR,
            self.country, // country VARCHAR,
            self.payload.activity_type,// activity_type VARCHAR,
            self.payload.url, // url VARCHAR,
            self.payload.order_total, // order_total Decimal(10,2),
            self.payload.order_id, // order_id VARCHAR,
            self.payload.element_id, // element_id VARCHAR,
            self.created_at.format("%Y-%m-%d %H:%M:%S"), // created_at TIMESTAMP,
            Utc::now().format("%Y-%m-%d")) // processed_at TIMESTAMP
    }
}

#[derive(Serialize)]

pub struct EventsResponse {
    pub ok: bool,
    pub error: Option<String>,
}
