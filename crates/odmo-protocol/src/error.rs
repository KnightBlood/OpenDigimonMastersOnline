use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("packet is too short")]
    PacketTooShort,
    #[error("invalid checksum for packet length {length}: expected {expected}, got {actual}")]
    InvalidChecksum {
        length: usize,
        expected: i16,
        actual: i16,
    },
    #[error("unexpected end of packet")]
    UnexpectedEof,
    #[error("invalid account packet type {0}")]
    InvalidAccountPacketType(i16),
    #[error("invalid character packet type {0}")]
    InvalidCharacterPacketType(i16),
    #[error("invalid game packet type {0}")]
    InvalidGamePacketType(i16),
    #[error("text field length {0} exceeds the 512 byte bound")]
    InvalidTextLength(usize),
    #[error("text field is not valid UTF-8")]
    InvalidTextEncoding,
    #[error("unsupported tamer summon discriminator {0}")]
    UnsupportedSummonCase(u8),
}
