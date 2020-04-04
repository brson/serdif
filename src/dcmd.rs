use crate::scmd;
use serde::Deserialize;

pub use scmd::SerializeBool;
pub use scmd::SerializeU8;

pub use scmd::SerializeTuple;
pub use scmd::SerializeTupleElement;

/*pub struct SerializeTupleElement<'de, T>
where T: Deserialize<'de>
{
    pub value: T,
    pub _phantom: PhantomData<&'de ()>,
}

impl<'de, T> Deserialize<'de> for SerializeTupleElement<'de, T>
where T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        Ok(SerializeTupleElement {
            value: T::deserialize(deserializer)?,
            _phantom: PhantomData,
        })
    }
}*/

pub use scmd::SerializeTupleEnd;

#[derive(Deserialize)]
pub struct SerializeStruct {
    pub name: String,
    pub len: usize,
}

#[derive(Deserialize)]
pub struct SerializeStructField {
    pub key: String,
}

pub use scmd::SerializeStructEnd;
