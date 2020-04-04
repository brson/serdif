use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializeBool {
    pub v: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeU8 {
    pub v: u8,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeTuple {
    pub len: usize,
}

#[derive(Serialize)]
pub struct SerializeTupleElement<'a, T>
where T: ?Sized + Serialize
{
    pub value: &'a T,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeTupleEnd;

#[derive(Serialize)]
pub struct SerializeStruct {
    pub name: &'static str,
    pub len: usize,
}

#[derive(Serialize)]
pub struct SerializeField<'a, T>
where T: ?Sized + Serialize
{
    pub key: &'static str,
    pub value: &'a T,
}

#[derive(Serialize, Deserialize)]
pub struct SerializeStructEnd;
