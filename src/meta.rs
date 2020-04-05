#![allow(unused)]

use anyhow::{bail, anyhow};
use crate::error::{Result, StdResultExt};
use serde::{Serialize, Deserialize};
use serde_json;
use byteorder::{ByteOrder, LittleEndian};
use hex;
use std::io::SeekFrom;
use crate::state::Buffer;

#[derive(Debug)]
pub struct Stitch {
    pub old_pos: u64,
    pub new_pos: u64,
    pub next_stitch_pos: u64,
}

impl Stitch {
    pub fn encode(self) -> FixedSizeStitch {
        let buf = &mut [0; 8];
        LittleEndian::write_u64(buf, self.old_pos);
        let old_pos = hex::encode(&buf);
        LittleEndian::write_u64(buf, self.new_pos);
        let new_pos = hex::encode(&buf);
        LittleEndian::write_u64(buf, self.next_stitch_pos);
        let next_stitch_pos = hex::encode(&buf);
        FixedSizeStitch {
            old_pos, new_pos, next_stitch_pos,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixedSizeStitch {
    old_pos: String,
    new_pos: String,
    next_stitch_pos: String,
}

impl FixedSizeStitch {
    pub fn decode(self) -> Result<Stitch> {
        let old_pos = hex::decode(&self.old_pos).e()?;
        let old_pos = LittleEndian::read_u64(&old_pos);
        let new_pos = hex::decode(&self.new_pos).e()?;
        let new_pos = LittleEndian::read_u64(&new_pos);
        let next_stitch_pos = hex::decode(&self.next_stitch_pos).e()?;
        let next_stitch_pos = LittleEndian::read_u64(&next_stitch_pos);
        Ok(Stitch {
            old_pos, new_pos, next_stitch_pos,
        })
    }
}

pub const MAGIC: u64 = 0x84124f4c417733f8;

#[derive(Serialize, Deserialize, Debug)]
pub struct Trailer {
    pub magic: u64,
    pub first_stitch: Option<u64>,
    pub prev_trailer_pos: Option<u64>,
}

fn find_last_trailer(buf: &mut dyn Buffer) -> Result<Option<(Trailer, u64)>> {
    let orig_pos = buf.seek(SeekFrom::Current(0)).e()?;
    let pos = buf.seek(SeekFrom::End(0)).e()?;
    if pos == 0 {
        return Ok(None);
    }

    for i in 0..1000 {
        let pos = buf.seek(SeekFrom::End(i)).e()?;
        let mut de = serde_json::Deserializer::from_reader(&mut *buf);
        let t = Trailer::deserialize(&mut de);
        if let Ok(t) = t {
            if t.magic == MAGIC {
                buf.seek(SeekFrom::Start(orig_pos)).e()?;
                return Ok(Some((t, pos)));
            }
        }
    }

    buf.seek(SeekFrom::Start(orig_pos)).e()?;
    Err(anyhow!("unable to find trailer block").into())
}

