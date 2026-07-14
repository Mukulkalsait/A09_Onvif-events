use common::camera::CameraCapibilities;
use pretty_assertions::assert_eq;

#[test]
fn test_cam_capibilities_creation() {
    let cam1 = common::camera::CameraCapibilities {
        pan_tilt_zoom: true,
        events: true,
        audio: false,
        imaging: true,
        media: true,
        ptz_absolute: Some(false),
        ptz_relative: Some(true),
        ptz_continous: Some(true),
        event_types: Some(vec!["Motion".to_string(), "Audio".to_string()]),
    };

    assert!(cam1.pan_tilt_zoom);
    assert!(cam1.events);
    assert!(!cam1.audio);
    assert!(cam1.imaging);
    assert!(!cam1.ptz_absolute.unwrap());
    assert_eq!(cam1.event_types, Some(vec!["Motion".to_string(), "Audio".to_string()]));
}

#[test]
fn test_cam_capibilities_default() { let camera_def = CameraCapibilities::default(); }
