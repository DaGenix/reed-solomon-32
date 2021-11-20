enum UsageErrorCode {
    InvalidEcc,
    InvalidDataLen,
    InvalidMessageLenForEcc,
    InvalidCombinedLen,
    InvalidSymbol,
    InvalidErasePos,
}

pub struct UsageErrorMessage {
    error_code: UsageErrorCode,
}

impl core::fmt::Debug for UsageErrorMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.error_code {
            UsageErrorCode::InvalidEcc =>
                write!(f, "The number of Ecc symbols must be less than 31."),
            UsageErrorCode::InvalidDataLen =>
                write!(f, "The length of the input data or message is greater than 31 symbols."),
            UsageErrorCode::InvalidMessageLenForEcc =>
                write!(f, "The message buffer is shorter than the number of ECC symbols and thus cannot be valid."),
            UsageErrorCode::InvalidCombinedLen =>
                write!(f, "The combination of data and ECC symbols would create a message greater than the maximum of 31 symbols."),
            UsageErrorCode::InvalidSymbol =>
                write!(f, "Invalid symbol. All symbols must be be in the range [0, 31]."),
            UsageErrorCode::InvalidErasePos =>
                write!(f, "One of the erasure positions was greater than the message size."),
        }
    }
}

impl core::fmt::Display for UsageErrorMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// An error that indicates that a parameter that was supplied to a function
/// was invalid for that function.
///
/// Details on the exact error may be obtained by formatting the value.
pub struct UsageError(pub UsageErrorMessage);

impl core::fmt::Debug for UsageError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Usage error: {}", self.0)
    }
}

impl core::fmt::Display for UsageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UsageError { }

pub fn invalid_ecc() -> UsageError {
    UsageError(UsageErrorMessage { error_code: UsageErrorCode::InvalidEcc })
}

pub fn invalid_data_len() -> UsageError {
    UsageError(UsageErrorMessage { error_code: UsageErrorCode::InvalidDataLen })
}

pub fn invalid_data_len_for_ecc() -> UsageError {
    UsageError(UsageErrorMessage { error_code: UsageErrorCode::InvalidMessageLenForEcc })
}

pub fn invalid_combined_len() -> UsageError {
    UsageError(UsageErrorMessage { error_code: UsageErrorCode::InvalidCombinedLen })
}

pub fn invalid_symbol() -> UsageError {
    UsageError(UsageErrorMessage { error_code: UsageErrorCode::InvalidSymbol })
}

pub fn invalid_erase_pos() -> UsageError {
    UsageError(UsageErrorMessage { error_code: UsageErrorCode::InvalidErasePos })
}

/// And error occurred while attempting to correct a message.
pub enum CorrectionError {
    /// The message had too many errors and they could not be corrected
    TooManyErrors,

    /// An invalid parameter value was passed to the function. Format the
    /// `UsageErrorMessage` in order to get more details about the error.
    UsageError(UsageErrorMessage),
}

impl From<UsageError> for CorrectionError {
    fn from(UsageError(message): UsageError) -> CorrectionError {
        CorrectionError::UsageError(message)
    }
}

impl core::fmt::Debug for CorrectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CorrectionError::TooManyErrors =>
                write!(f, "The message has too many errors and cannot be repaired"),
            CorrectionError::UsageError(err) =>
                write!(f, "Usage error: {}", err),
        }
    }
}

impl core::fmt::Display for CorrectionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CorrectionError { }
