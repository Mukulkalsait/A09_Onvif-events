use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Struct: CameraID
/// unique camera id
/// PartialEq, and Eq for comparision.
pub struct CameraID(pub String);

impl CameraID {
    /// # Camera ID COnstrctor
    /// ```
    /// let id = common::CameraID::new("cameraxyz");
    /// assert_eq!(ia.as_str(),"cameraxyz" );
    /// ```
    pub fn new(id: impl Into<String>) -> Self { Self(id.into()) }

    /// # Return Camera-ID as &str slice
    /// use: When you need to read the value, e.g., passing to a function that needs &str
    pub fn as_str(&self) -> &str { &self.0 }

    /// Consumes Camera-Id and return inner String
    /// use: When you need ownership of the String, e.g., moving to another system
    pub fn into_inner(self) -> String { self.0 }
}

impl fmt::Display for CameraID {
    /// use: For UI display, logs, user messages
    /// println!("Camera: {}", id); // prints "Camera: cam1"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<String> for CameraID {
    /// Implicit conversion from strings => use: When you want automatic conversion
    /// let id: CameraId = String::from("cam1").into();  // From<String>
    fn from(s: String) -> Self { Self(s) }
}

impl From<&str> for CameraID {
    /// Implicit conversion from &str => use: When you want automatic conversion
    /// let id: CameraId = "cam1".into();  // From<&str>
    fn from(s: &str) -> Self { Self(s.to_string()) }
}

// Impl: Special one for AsRef
impl AsRef<str> for CameraID {
    ///  Generic borrowing pattern => use: When you want a function to accept anything that can be referenced as &str
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// Struct: CameraInfo
pub struct CameraInfo {
    pub manufacturer: String,
    pub model: String,
    pub firmware_version: String,
    pub serial_number: String,
    pub hardware_id: Option<String>,
}

impl CameraInfo {
    pub fn new(manufacturer: String, model: String, firmware_version: String, serial_number: String) -> Self {
        CameraInfo { manufacturer, model, firmware_version, serial_number, hardware_id: None }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
/// Struct: CamCapibilities
///
///
pub struct CameraCapibilities {
    pub pan_tilt_zoom: bool,
    pub events: bool, // IMP: events Support
    pub audio: bool,
    pub imaging: bool, // Image Settings (brightness, contrast)
    pub media: bool,
    pub ptz_absolute: Option<bool>,
    pub ptz_relative: Option<bool>,
    pub ptz_continous: Option<bool>,
    pub event_types: Option<Vec<String>>,
}

impl CameraCapibilities {
    /// should return everyting false for default
    pub fn new() -> Self { Self::default() }
}
