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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserEvent {
    pub id: String, // user id
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize)]
pub struct Segment {
    pub id: u16, // user id
    pub title: String,
    pub description: String,
    pub tag: String,
    //snake case
    #[serde(rename = "whereStatement")]
    pub where_statement: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SegmentsResponse {
    pub data: Vec<Segment>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserSegment {
    pub user_id: u16,
    pub segment_id: u16,
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub created_at: DateTime<Utc>,
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub id: u16, // user id
    pub name: String,
    pub email: String,
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub created_at: DateTime<Utc>,
    #[serde(
        serialize_with = "date_serialize",
        deserialize_with = "date_deserialize"
    )]
    pub updated_at: DateTime<Utc>,
    pub segments: Vec<UserSegment>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GetUserResponse {
    pub data: User,
}
