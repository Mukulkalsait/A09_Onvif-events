use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use crate::camera::CameraID;

#[derive(Debug, Deserialize, Serialize)]
/// Struct: OnvifError
/// Onvif Error Types
pub enum OnvifError {
    NetworkError {
        message: String,
        camera_id: Option<CameraID>,
    },
    AuthenticationError {
        message: String,
        camera_id: Option<CameraID>,
    },
    NotFound {
        message: String,
        camera_id: Option<CameraID>,
    },

    /// ### Contains different field source.
    XmlError {
        message: String,
        source: Option<String>,
    },
    /// ### Contains expected vs recieved responses fields.
    InvalidResponse {
        message: String,
        expected: Option<String>,
        recieved: Option<String>,
    },
    /// ### Contains Additional Field features.
    UnsupportedFeature {
        message: String,
        feature: String,
        camera_id: Option<CameraID>,
    },
    /// ### Contains additional field duration.
    Timeout {
        message: String,
        duration_ms: Option<u64>,
        camera_id: Option<CameraID>,
    },
}

// Impl: Display for OnvifError
impl fmt::Display for OnvifError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OnvifError::NetworkError { message, camera_id } => {
                if let Some(id) = camera_id {
                    write!(f, "Network Error: {}, camera: {}", message, id)
                } else {
                    write!(f, "Network Error: {}", message)
                }
            }
            OnvifError::AuthenticationError { message, camera_id } => {
                if let Some(id) = camera_id {
                    write!(f, "Authentication Error: {}, camera: {}", message, id)
                } else {
                    write!(f, "Authentication Error: {}", message)
                }
            }
            OnvifError::NotFound { message, camera_id } => {
                if let Some(id) = camera_id {
                    write!(f, "Not Found: {}, camera: {}", message, id)
                } else {
                    write!(f, "Not Found: {}", message)
                }
            }

            OnvifError::XmlError { message, source } => {
                if let Some(source) = source {
                    write!(f, "XML Error: {}, source: {}", message, source)
                } else {
                    write!(f, "XML Error: {}", message)
                }
            }
            OnvifError::InvalidResponse { message, expected, recieved } => {
                if let (Some(exp), Some(rec)) = (expected, recieved) {
                    write!(f, "Invalid Response: {}, (expected: {}| recieved: {})", message, exp, rec)
                } else {
                    write!(f, "Invalid Response: {}", message)
                }
            }
            OnvifError::UnsupportedFeature { message, feature, camera_id } => {
                if let Some(id) = camera_id {
                    write!(f, "Unsupported Feature: {} - {}, camera: {}", message, feature, id)
                } else {
                    write!(f, "Unsupported Feature: {} - {}", message, feature)
                }
            }
            OnvifError::Timeout { message, duration_ms, camera_id } => {
                if let (Some(ms), Some(id)) = (duration_ms, camera_id) {
                    write!(f, "Timeout: {} after {}ms, camera: {}", message, ms, id)
                } else if let Some(ms) = duration_ms {
                    write!(f, "Timeout: {} after {}ms", message, ms,)
                } else if let Some(id) = camera_id {
                    write!(f, "Timeout: {}, camera: {}", message, id)
                } else {
                    write!(f, "Timeout: {}", message)
                }
            }
        }
    }
}

// IMPL: Error for OnvifError
//
//  # now Rust know OnvifError is Error type 💫💫💫💫💫
//    - ? can be use with OnvifError.
///    ```
///       let response = request::get(url).await?; // Y:
///
///        // Now this works!
///       impl From<reqwest::Error> for OnvifError {
///            fn from(err: reqwest::Error) -> Self {
///                OnvifError::NetworkError { message: err.to_string() }
///            }
///        }
///        fn get_camera() -> Result<CameraInfo> {
///            let response = reqwest::get(url).await?;  // auto conversion
///        }
///    ```
//    - Error handlers know it now.
//    - Box<dyn Error> know it now.
//    - i have checked methods like {source, cause } can be usefull later
//
impl Error for OnvifError {}

/// # Specific Result Type for Onvif Error TYPE: Result<T, OnvifError>
/// ```
///  // now we use
///  fn get_camera() -> Result<CameraInfo> {...}
///  // instead of
///  fn get_camera() -> Result<CameraInfo, OnvifError>{...}
/// ```
/// >   ### "?" can be use on it directly now.
pub type Result<T> = std::result::Result<T, OnvifError>;
