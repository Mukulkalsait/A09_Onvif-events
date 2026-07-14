use common::camera::CameraInfo;
use pretty_assertions::assert_eq;

#[test]
fn test_cam_info_creation() {
    let info = CameraInfo::new("Hikvision".to_string(), "DS-2CD2043G0-I".to_string(), "V5.5.82".to_string(), "123456789".to_string());

    assert_eq!(info.manufacturer, "Hikvision");
    assert_eq!(info.model, "DS-2CD2043G0-I");
    assert_eq!(info.firmware_version, "V5.5.82");
    assert_eq!(info.serial_number, "123456789");
    assert!(info.hardware_id.is_none());
}
