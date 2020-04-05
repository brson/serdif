#![allow(unused)]

use crate::error::{Result, StdResultExt};
use serde::{Serialize, Deserialize};
use byteorder::{ByteOrder, LittleEndian};
use hex;

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

#[derive(Serialize, Deserialize)]
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
