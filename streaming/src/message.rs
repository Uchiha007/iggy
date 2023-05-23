#[derive(Debug, Clone)]
pub struct Message {
    pub offset: u64,
    pub timestamp: u64,
    pub length: u32,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn empty(timestamp: u64, payload: Vec<u8>) -> Self {
        Message::create(0, timestamp, payload)
    }

    pub fn create(offset: u64, timestamp: u64, payload: Vec<u8>) -> Self {
        Message {
            offset,
            timestamp,
            length: payload.len() as u32,
            payload,
        }
    }

    pub fn get_size_bytes(&self) -> u32 {
        // Offset + Timestamp + Length + Payload
        8 + 8 + 4 + self.length
    }

    pub fn extend(&self, bytes: &mut Vec<u8>) {
        bytes.extend(self.offset.to_le_bytes());
        bytes.extend(self.timestamp.to_le_bytes());
        bytes.extend(self.length.to_le_bytes());
        bytes.extend(&self.payload);
    }
}
