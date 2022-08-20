use crate::census::Census;
use derive_more::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

macro_rules! impl_nation {
    (
        $(#[$($attribute:meta)+])*
        $struct_vis:vis struct $name:ident {
            $(
                $(#[serde(rename = $lit:literal)])?
                $field_vis:vis $field_name:ident: $field_type:ty,
            )*
        }
    ) => {
        $(#[$($attribute)+])*
        $struct_vis struct $name{
            #[serde(flatten)]
            pub fields: HashMap<String, XmlData>
        }
        impl $name{
            $(
                #[allow(dead_code)]
                $field_vis fn $field_name(&self) -> Option<$field_type>{
                    let _name = stringify!($field_name);
                    impl_nation!{@if $field_name == founded{
                        let s:Option<String> = self.fields.get("founded").and_then(|s|s.clone().try_into().ok());
                        s.or_else(||self.fields.get("founded").map(|n|{
                            let n:u64 = n.clone().try_into().unwrap();
                            n.to_string()
                        })).map(|s|s.try_into().unwrap())

                    } else {
                        let name = impl_nation!(@if $($lit)? {($($lit)?).to_owned()} else {_name.to_uppercase()});
                        self.fields.get(&name).and_then(|s|s.clone().try_into().ok())
                    }
                }}
            )*
        }
    };
    (@if $cond:literal $yes:block else $no:block) => {
        $yes
    };
    (@if $l:ident == $r:ident $yes:block else $no:block) => {
        macro_rules! __helper{
            ($l $l) => {
                $yes
            };
            ($l $r) => {
                $no
            };
        }
        __helper!($l $r)
    };
    (@if $yes:block else $no:block) => {
        $no
    };

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TryInto, Unwrap, IsVariant)]
#[serde(untagged)]
pub enum XmlData {
    Int(u64),
    Float(f64),
    Admirable(Admirable),
    Admirables(Vec<Admirable>),
    Banner(Banner),
    Banners(Vec<Banner>),
    Causes(Vec<Cause>),
    Census(Census),
    Dispatches(Vec<Dispatch>),
    Factbooks(Vec<Factbook>),
    String(String),
}
impl_nation! {
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Nation {
    #[serde(rename = "id")]
    pub id: String,
    pub admirable: Admirable,
    pub admirables: Vec<Admirable>,
    pub animal: String,
    #[serde(rename = "ANIMALTRAIT")]
    pub animal_trait: String,
    #[serde(rename = "ISSUESANSWERED")]
    pub answered: u64,
    pub banner: Banner,
    pub banners: Vec<Banner>,
    pub capital: String,
    pub category: String,
    pub census: Census,
    pub crime: String,
    pub currency: String,
    pub leader: String,
    pub religion: String,
    pub dbid: u64,
    pub deaths: Vec<Cause>,
    pub demonym: String,
    pub demonym2: String,
    #[serde(rename = "DEMONYM2PLURAL")]
    pub demonym_2_plural: String,
    pub dispatches: u64,
    #[serde(rename = "DISPATCHLIST")]
    pub dispatch_list: Vec<Dispatch>,
    pub endorsements: String,
    pub factbooks: u64,
    #[serde(rename = "FACTBOOKLIST")]
    pub factbook_list: Vec<Factbook>,
    #[serde(rename = "FIRSTLOGIN")]
    pub first_login: u64,
    pub flag: String,
    pub founded: String,
    #[serde(rename = "FOUNDEDTIME")]
    pub founded_time: u64,
}
}
