//! Special data types that will be generated directly, without parsing.

pub const DATA_TYPES: [&str; 13] = [
    "Base64Datatype",
    "BooleanDatatype",
    "DateDatatype",
    "DateTimeWithTimezoneDatatype",
    "EmailAddressDatatype",
    "IntegerDatatype",
    "NonNegativeIntegerDatatype",
    "PositiveIntegerDatatype",
    "StringDatatype",
    "TokenDatatype",
    "URIDatatype",
    "URIReferenceDatatype",
    "UUIDDatatype",
];

pub fn is_datatype(value: &str) -> bool {
    DATA_TYPES.contains(&value)
}
