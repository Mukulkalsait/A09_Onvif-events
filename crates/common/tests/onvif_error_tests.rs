use common::camera::CameraID;
use common::onvif_error::{self, OnvifError};
use test_case::test_case;

const TEST_CAMERA_ID: &str = "myCam-123";

fn test_cam(present: bool) -> Option<CameraID> {
    match present {
        false => None,
        true => Some(CameraID::new(TEST_CAMERA_ID)),
    }
}

#[test_case( OnvifError::NetworkError { message: "no camera detected".to_string(), camera_id: test_cam(true) }, "Network Error: no camera detected, camera: myCam-123")]
#[test_case( OnvifError::AuthenticationError { message: "invalid username".to_string(), camera_id: test_cam(true)}, "Authentication Error: invalid username, camera: myCam-123")]
#[test_case( OnvifError::NotFound { message: "module not found".to_string(), camera_id: test_cam(false)}, "Not Found: module not found")]
/// Modified compact tests.
fn test_onvif_error_display(err: OnvifError, expected: &str) {
    assert_eq!(format!("{}", err), expected);
}

#[test_case("wrong format".to_string(), None, "XML Error: wrong format".to_string())]
#[test_case("wrong format".to_string(), Some("invalid tag: <notag>".to_string()), "XML Error: wrong format, source: invalid tag: <notag>".to_string())]
fn test_onvif_error_display_xmlerror(message: String, source: Option<String>, res: String) {
    let err = onvif_error::OnvifError::XmlError { message, source };
    assert_eq!(format!("{}", err), res)
}

#[test_case("missing fields".to_string(), None, None, "Invalid Response: missing fields".to_string())]
#[test_case("missing fields".to_string(), Some("manufracturer".to_string()), None, "Invalid Response: missing fields".to_string())]
fn test_onvif_error_display_invalidresponse(message: String, expected: Option<String>, recieved: Option<String>, res: String) {
    let err = onvif_error::OnvifError::InvalidResponse { message, expected, recieved };
    assert_eq!(format!("{}", err), res)
}

#[test_case("not available".to_string(),"PTZ".to_string(),test_cam(true), "Unsupported Feature: not available - PTZ, camera: myCam-123".to_string() )]
#[test_case("not available".to_string(),"PTZ".to_string(),test_cam(false), "Unsupported Feature: not available - PTZ".to_string() )]
fn test_onvif_error_display_unsuportedfeatures(message: String, feature: String, camera_id: Option<CameraID>, res: String) {
    let err = onvif_error::OnvifError::UnsupportedFeature { message, feature, camera_id };
    assert_eq!(format!("{}", err), res)
}

#[test_case("operation timed out".to_string(), Some(345),test_cam(true),"Timeout: operation timed out after 345ms, camera: myCam-123".to_string() )]
#[test_case("operation timed out".to_string(), Some(345),test_cam(false),"Timeout: operation timed out after 345ms".to_string() )]
#[test_case("operation timed out".to_string(), None,test_cam(true),"Timeout: operation timed out, camera: myCam-123".to_string() )]
#[test_case("operation timed out".to_string(), None,test_cam(false),"Timeout: operation timed out".to_string() )]
fn test_onvif_error_display_timeout(message: String, duration_ms: Option<u64>, camera_id: Option<CameraID>, res: String) {
    let err = onvif_error::OnvifError::Timeout { message, duration_ms, camera_id };
    assert_eq!(format!("{}", err), res)
}
