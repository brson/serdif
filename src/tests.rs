use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::io::Cursor;
use serdif::Serializer;

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::<u8>::new())
}

#[test]
fn test_u8() -> Result<()> {
    let buf = buffer();
    let mut ser = Serializer::new(buf);

    let val1 = 5u8;
    val1.serialize(&mut ser)?;

    let mut de = ser.to_de();
    de.reset()?;

    let val2 = u8::deserialize(&mut de)?;

    assert_eq!(val1, val2);

    Ok(())
}

#[test]
fn test_bool() -> Result<()> {
    let buf = buffer();
    let mut ser = Serializer::new(buf);

    let val1 = true;
    val1.serialize(&mut ser)?;

    let mut de = ser.to_de();
    de.reset()?;

    let val2 = bool::deserialize(&mut de)?;

    assert_eq!(val1, val2);

    Ok(())
}

#[test]
fn test_tuple() -> Result<()> {
    let buf = buffer();
    let mut ser = Serializer::new(buf);

    let val1 = (true, false);
    val1.serialize(&mut ser)?;

    let mut de = ser.to_de();
    de.reset()?;

    let val2 = <(bool, bool)>::deserialize(&mut de)?;

    assert_eq!(val1, val2);

    Ok(())
}

#[test]
fn test_struct() -> Result<()> {
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
    de.reset()?;

    let val2 = Type1::deserialize(&mut de)?;

    assert_eq!(val1, val2);

    Ok(())
}

#[test]
fn test_overwrite_tuple() -> Result<()> {
    let buf = buffer();
    let mut ser = Serializer::new(buf);

    let val1 = (true, false);
    val1.serialize(&mut ser)?;

    ser.reset()?;

    let val2 = (false, true);
    val2.serialize(&mut ser)?;

    let mut de = ser.to_de();
    de.reset()?;

    let val3 = <(bool, bool)>::deserialize(&mut de)?;

    assert_eq!(val2, val3);

    Ok(())
}
