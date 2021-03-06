use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SerializeBool {
    pub v: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeU8 {
    pub v: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SerializeTuple {
    pub len: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SerializeTupleElement;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SerializeTupleEnd;

#[derive(Serialize)]
pub struct SerializeStruct {
    pub name: &'static str,
    pub len: usize,
}

#[derive(Serialize)]
pub struct SerializeStructField {
    pub key: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeStructEnd;
