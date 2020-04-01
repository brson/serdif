use serde::Serialize;

#[derive(Serialize)]
pub struct SerializeStruct {
    pub name: &'static str,
    pub len: usize,
}
