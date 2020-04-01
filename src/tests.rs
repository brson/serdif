use serde::Serialize;
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

    #[derive(Serialize)]
    struct Type1 {
        field1: bool,
        field2: bool,
    }

    let val = Type1 { field1: true, field2: false };
    val.serialize(&mut ser)?;

    Ok(())
}
