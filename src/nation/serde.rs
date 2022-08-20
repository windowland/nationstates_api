use serde::{Deserialize, Deserializer, Serialize};

use crate::census::Census;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Nation {
    #[serde(rename = "id")]
    pub id: String,
    pub admirable: Option<Admirable>,
    pub admirables: Option<Vec<Admirable>>,
    pub animal: Option<String>,
    #[serde(rename = "ANIMALTRAIT")]
    pub animal_trait: Option<String>,
    #[serde(rename = "ISSUESANSWERED")]
    pub answered: Option<u64>,
    pub banner: Option<Banner>,
    pub banners: Option<Vec<Banner>>,
    pub capital: Option<String>,
    pub category: Option<String>,
    pub census: Option<Census>,
    pub crime: Option<String>,
    pub currency: Option<String>,
    pub leader: Option<String>,
    pub religion: Option<String>,
    pub dbid: Option<u64>,
    pub deaths: Option<Vec<Cause>>,
    pub demonym: Option<String>,
    pub demonym2: Option<String>,
    #[serde(rename = "DEMONYM2PLURAL")]
    pub demonym_2_plural: Option<String>,
    pub dispatches: Option<u64>,
    #[serde(rename = "DISPATCHLIST")]
    pub dispatch_list: Option<Vec<Dispatch>>,
    #[serde(deserialize_with = "deserialize_endorsements")]
    pub endorsements: Option<Vec<String>>,
    pub factbooks: Option<u64>,
    #[serde(rename = "FACTBOOKLIST")]
    pub factbook_list: Option<Vec<Factbook>>,
    #[serde(rename = "FIRSTLOGIN")]
    pub first_login: Option<u64>,
    pub flag: Option<String>,
    pub founded: Option<String>,
    #[serde(rename = "founded_time")]
    pub founded_time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Admirable(pub String);

#[test]
fn parse_admirable() {
    let input = r#"<ADMIRABLE>test</ADMIRABLE>"#;
    let expected = Admirable("test".to_string());
    let actual = quick_xml::de::from_str::<Admirable>(input).unwrap();
    assert_eq!(expected, actual);
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Banner(pub String);

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub struct Cause {
    #[serde(rename = "type")]
    pub reason: String,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Dispatch {
    #[serde(rename = "id")]
    pub id: u64,
    pub title: String,
    pub author: String,
    pub category: String,
    pub subcategory: String,
    pub created: u64,
    pub edited: u64,
    pub views: u64,
    pub score: u64,
}

fn deserialize_endorsements<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<Vec<String>>, D::Error> {
    Option::<String>::deserialize(de).map(|x| x.map(|x| x.split(",").map(str::to_owned).collect()))
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Factbook {
    #[serde(rename = "id")]
    pub id: u64,
    pub title: String,
    pub author: String,
    pub category: String,
    pub subcategory: String,
    pub created: u64,
    pub edited: u64,
    pub views: u64,
    pub score: u64,
}
