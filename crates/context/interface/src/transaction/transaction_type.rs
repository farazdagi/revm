//! Transaction type enum.

/// Transaction types of all Ethereum transaction
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TransactionType {
    /// Legacy transaction type
    Legacy = 0,
    /// EIP-2930 Access List transaction type
    Eip2930 = 1,
    /// EIP-1559 Fee market change transaction type
    Eip1559 = 2,
    /// EIP-4844 Blob transaction type
    Eip4844 = 3,
    /// EIP-7702 Set EOA account code transaction type
    Eip7702 = 4,
    /// Custom type means that the transaction trait was extended and has custom types
    Custom = 0xFF,
}

impl TransactionType {
    /// Returns true if the transaction type is legacy.
    pub fn is_legacy(&self) -> bool {
        matches!(self, Self::Legacy)
    }

    /// Returns true if the transaction type is custom.
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom)
    }
}

impl PartialEq<u8> for TransactionType {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == *other
    }
}

impl PartialEq<TransactionType> for u8 {
    fn eq(&self, other: &TransactionType) -> bool {
        *self == (*other as u8)
    }
}

impl From<TransactionType> for u8 {
    fn from(tx_type: TransactionType) -> u8 {
        tx_type as u8
    }
}

impl From<u8> for TransactionType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Legacy,
            1 => Self::Eip2930,
            2 => Self::Eip1559,
            3 => Self::Eip4844,
            4 => Self::Eip7702,
            _ => Self::Custom,
        }
    }
}
