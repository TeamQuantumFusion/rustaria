use serde::{Deserialize, Serialize};

use rustaria_api::ty::RawId;
use rustaria_util::ty::pos::Pos;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ServerEntityPacket {
    Spawn(RawId, Pos)
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ClientEntityPacket {
 
}