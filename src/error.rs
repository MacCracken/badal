use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BadalError {
    #[error("invalid temperature: {0}")]
    InvalidTemperature(String),
    #[error("invalid pressure: {0}")]
    InvalidPressure(String),
    #[error("invalid humidity: {0}")]
    InvalidHumidity(String),
    #[error("invalid altitude: {0}")]
    InvalidAltitude(String),
    #[error("computation error: {0}")]
    ComputationError(String),
}

pub type Result<T> = std::result::Result<T, BadalError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let e = BadalError::InvalidTemperature("below absolute zero".into());
        assert!(e.to_string().contains("below absolute zero"));
    }

    #[test]
    fn result_type() {
        let ok: Result<f64> = Ok(1.0);
        assert!(ok.is_ok());
    }
}
