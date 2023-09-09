#[derive(Debug)]
pub enum FError {
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
    DeflateError(String),
    NoWorld,
    NoCampaign,
    UnknownWorldSize,
    StreamOverflow(usize, usize, usize),
    NoZeroTerminator,
    EntityNoESH,
    NoESHValue,
    ESHValueNonBinary,
    AttributesNonBinary,
    ValueNoESBIN,
}

impl std::fmt::Display for FError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FError as FE;
        match self {
            FE::IOError(e) => write!(f, "IOError {}", e),
            FE::Utf8Error(e) => write!(f, "Utf8Error {}", e),
            FE::DeflateError(e) => write!(f, "DeflateError {}", e),
            FE::NoWorld => write!(f, "No world found in file"),
            FE::NoCampaign => write!(f, "No campaign found after world"),
            FE::UnknownWorldSize => write!(f, "Unable to determine world block size"),
            FE::StreamOverflow(offset, size, read) => write!(
                f,
                "stream read {} at offset {} overflow size {}",
                read, offset, size
            ),
            FE::NoZeroTerminator => write!(f, "No zero-terminator when String::decode"),
            FE::EntityNoESH => write!(f, "Entity has no ESH"),
            FE::NoESHValue => write!(f, "Entity has no specific ESH value"),
            FE::ESHValueNonBinary => write!(f, "ESH value is not binary"),
            FE::AttributesNonBinary => write!(f, "Attributes Binary != true"),
            FE::ValueNoESBIN => write!(f, "Value has no esbin"),
        }
    }
}

impl From<std::io::Error> for FError {
    fn from(value: std::io::Error) -> Self {
        FError::IOError(value)
    }
}

impl From<std::str::Utf8Error> for FError {
    fn from(value: std::str::Utf8Error) -> Self {
        FError::Utf8Error(value)
    }
}
