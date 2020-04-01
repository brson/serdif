use serde::Serialize;

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

#[derive(Serialize)]
pub struct SerializeStructEnd;
