use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::io::Cursor;
use serdif::Serializer;

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::<u8>::new())
}

#[test]
fn test() -> Result<()> {
    let buf = buffer();
    let mut ser = Serializer::new(buf);

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Type1 {
        field1: bool,
        field2: bool,
    }

    let val1 = Type1 { field1: true, field2: false };
    val1.serialize(&mut ser)?;

    let mut de = ser.to_de();

    let val2 = Type1::deserialize(&mut de)?;

    assert_eq!(val1, val2);

    Ok(())
}
