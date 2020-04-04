use crate::scmd;
use serde::Deserialize;

pub use scmd::SerializeU8;

#[derive(Deserialize)]
pub struct SerializeStruct {
    pub name: String,
    pub len: usize,
}

/*#[derive(Deserialize)]
pub struct SerializeField<'a, T>
where T: ?Sized + Deserialize<'a>
{
    pub key: String,
    pub value: T,
}*/

#[derive(Deserialize)]
pub struct SerializeStructEnd;
