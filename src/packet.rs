use std::error::Error;

use bitvec::prelude::*;

pub const PACKET_PADDING_SIZE_BYTES: usize = 2;
pub const PACKET_HEADER_SIZE_BYTES: usize = 8;

pub fn min_packet_size_bytes() -> usize {
  PACKET_PADDING_SIZE_BYTES + PACKET_HEADER_SIZE_BYTES
}

pub fn max_packet_size_bytes() -> usize {
  min_packet_size_bytes() + 4096
}

pub enum PacketType {
  ServerDataAuth,
  ServerDataAuthResponse,
  ServerDataExecCommand,
  ServerDataResponseValue
}

pub fn packet_type_to_int(pt: PacketType) -> i32 {
  match pt {
    PacketType::ServerDataAuth => 3,
    PacketType::ServerDataAuthResponse | PacketType::ServerDataExecCommand => 2,
    PacketType::ServerDataResponseValue => 0
  }
}

pub fn str_to_packet_type(packet_type_as_str: &str) -> Result<PacketType, Box<dyn Error>> {
  match packet_type_as_str {
    "SERVER_DATA_AUTH" => Ok(PacketType::ServerDataAuth),
    "SERVER_DATA_AUTH_RESPONSE" => Ok(PacketType::ServerDataAuthResponse),
    "SERVER_DATA_EXEC_COMMAND" => Ok(PacketType::ServerDataExecCommand),
    "SERVER_DATA_RESPONSE_VALUE" => Ok(PacketType::ServerDataResponseValue),
    _ => Err("Invalid Packet Type")?
  }
}

#[derive(Debug)]
pub struct Packet {
  pub size: usize,
  pub id: i32,
  pub typ: i32,
  pub body: Vec<u8>,
}

impl Packet {
  pub fn new<'a>(packet_type: PacketType, packet_id: i32, body: String) -> Result<Self, &'a str> {
    let size = body.len() + PACKET_HEADER_SIZE_BYTES + PACKET_PADDING_SIZE_BYTES;
    let max = max_packet_size_bytes();

    if size > max {
      Err("Body is larger than 4096 bytes")?;
    }

    Ok(Packet {
      size,
      id: packet_id,
      typ: packet_type_to_int(packet_type),
      body: body.into_bytes(),
    })
  }

  pub fn to_bytes<'a>(packet: Packet) -> Result<Vec<u8>, &'a str> {
    let mut bytes: BitVec<u8, Lsb0> = BitVec::new();
  
    bytes.extend_from_bitslice(&packet.size.to_le_bytes().view_bits::<Lsb0>());
    bytes.extend_from_bitslice(&packet.id.to_le_bytes().view_bits::<Lsb0>());
    bytes.extend_from_bitslice(&packet.typ.to_le_bytes().view_bits::<Lsb0>());
  
    let zero_byte = u8::try_from(0x00).map_err(|_| "Failed to convert 0x00 to u8")?;
  
    bytes.extend_from_raw_slice(&packet.body);
    bytes.extend_from_bitslice([zero_byte, zero_byte].view_bits::<Lsb0>());
  
    Ok(bytes.into_vec())
  }

  pub fn from_bytes<'a>(bytes: Vec<u8>) -> Result<Packet, &'a str> {
    let min = min_packet_size_bytes();
    let max = max_packet_size_bytes();

    // Check if the length of the byte vector is within the valid range
    if bytes.len() < min || bytes.len() > max {
        return Err("Invalid byte vector length".into());
    }

    let size_bytes = bytes[..4].try_into().map_err(|_| "Error parsing size")?;
    let id_bytes = bytes[4..8].try_into().map_err(|_| "Error parsing id")?;
    let type_bytes = bytes[8..12].try_into().map_err(|_| "Error parsing type")?;

    let size = usize::from_le_bytes(size_bytes);
    let id = i32::from_le_bytes(id_bytes);
    let typ = i32::from_le_bytes(type_bytes);

    let body = bytes[12..bytes.len() - 2].to_vec();

    Ok(Packet { size, id, typ, body })
  }
}

