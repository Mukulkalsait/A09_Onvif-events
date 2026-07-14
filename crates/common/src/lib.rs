use serde::{Deserialize, Serialize};
use std::fmt::{self, write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

impl AsRef<str> for CameraID {
    ///  Generic borrowing pattern => use: When you want a function to accept anything that can be referenced as &str
    fn as_ref(&self) -> &str { &self.0 }
}
