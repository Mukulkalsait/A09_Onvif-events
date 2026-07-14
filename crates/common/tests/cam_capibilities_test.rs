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
fn test_cam_capibilities_default() {
    let camera_def = CameraCapibilities::default();

    assert!(!camera_def.pan_tilt_zoom);
    assert!(!camera_def.events);
    assert!(!camera_def.audio);
    assert!(!camera_def.imaging);
    assert!(!camera_def.media);

    assert_eq!(camera_def.ptz_absolute, None);
    assert_eq!(camera_def.ptz_relative, None);
    assert_eq!(camera_def.ptz_continous, None);
    assert_eq!(camera_def.event_types, None);
}

#[test]
fn test_cam_capibilities_new() {
    let cam_new = CameraCapibilities::new();

    assert!(!cam_new.pan_tilt_zoom);
    assert!(!cam_new.events);
}

#[test]
fn test_cam_capibilities_clone() {
    let cam1 = CameraCapibilities {
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

    let cam2 = cam1.clone();
    assert_eq!(cam1, cam2);
}

#[test]
fn test_cam_capibilities_seralize_deseralize() -> Result<(), serde_json::Error> {
    let cam_ser = CameraCapibilities {
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

    let json = serde_json::to_string(&cam_ser)?; // Y: Seralization
    let parsed: CameraCapibilities = serde_json::from_str(&json)?; // Y:  De-Seralization

    assert_eq!(cam_ser, parsed);
    Ok(())
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
