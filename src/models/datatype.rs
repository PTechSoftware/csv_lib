use std::fmt::{Display, Formatter};

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
#[allow(dead_code)]
/// ## DataType Enum
/// Holds the datatypes most commons.
/// Allows to define the expected datatype.
pub enum DataType {
    TextRef,
    Text,
    Byte,
    UByte,
    Short,
    UShort,
    Integer,
    UInteger,
    Long,
    ULong,
    Float,
    Double,
    Boolean,
    Custom,
    Empty,
    AutoDetect
}

/// ## Display implementation
/// - Implement the fmt function for the trait.
impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::TextRef => {write!(f, "TextRef")}
            DataType::Text => {write!(f, "Text")}
            DataType::Byte => {write!(f, "Byte")}
            DataType::UByte => {write!(f, "UByte")}
            DataType::Short => {write!(f, "Short")}
            DataType::UShort => {write!(f, "UShort")}
            DataType::Integer => {write!(f, "Integer")}
            DataType::UInteger => {write!(f, "UInteger")}
            DataType::Long => {write!(f, "Long")}
            DataType::ULong => {write!(f, "ULong")}
            DataType::Float => {write!(f, "Float")}
            DataType::Double => {write!(f, "Double")}
            DataType::Boolean => {write!(f, "Boolean")}
            DataType::Custom => {write!(f, "Custom")}
            DataType::Empty => {write!(f, "Empty")}
            DataType::AutoDetect => {write!(f, "AutoDetect")}
        }
    }
}