use common::cameraid::CameraID;
use pretty_assertions::assert_eq;

#[test]
fn test_cam_id_creation() {
    let id = CameraID::new("cam1");
    assert_eq!(id.0, "cam1");
}

#[test]
fn test_cam_id_from_string() {
    let id: CameraID = "cam2".into();
    assert_eq!(id.0, "cam2");
}

#[test]
fn test_cam_id_clone() {
    let id = CameraID::new("cam1");
    let cloned = id.clone();
    assert_eq!(id.0, cloned.0);
    assert_eq!(id, cloned);
}

#[test]
fn test_cam_id_display() {
    let id = CameraID::new("camDisplay");
    assert_eq!(format!("{}", id), "camDisplay");
}

#[test]
fn test_cam_id_debug() {
    let id = CameraID::new("camid");
    assert_eq!(format!("{:?}", id), "CameraID(\"camid\")")
}

#[test]
fn test_cam_id_into_inner() {
    let id = CameraID::new("cam6");
    assert_eq!(id.into_inner(), "cam6".to_string());
}

#[test]
fn test_cam_id_ref() {
    let id = CameraID::new("camRef");
    let s: &str = id.as_ref();
    assert_eq!(s, "camRef");
}

#[test]
fn test_cam_id_from_str() {
    let id = CameraID::new("camStr");
    assert_eq!(id.as_str(), "camStr");
}

#[test]
fn test_cam_id_from_string_owned() {
    let s = String::from("camString");
    let id: CameraID = s.into();
    assert_eq!(id.as_str(), "camString");
}

#[test]
fn test_cam_id_new_vs_from() {
    let id1: CameraID = CameraID::new("cam10");
    let id2: CameraID = "cam10".into();
    assert_eq!(id1, id2);
}

#[test]
fn test_cam_id_hash() {
    use std::collections::HashSet;

    let mut set: HashSet<CameraID> = HashSet::new();
    let id1 = CameraID::new("cam11");
    let id2 = CameraID::new("cam11"); // duplicate
    let id3 = CameraID::new("cam12");
    let id4 = CameraID::new("cam13");

    set.insert(id1);
    set.insert(id2);
    set.insert(id3);
    set.insert(id4);

    assert_eq!(set.len(), 3); // 3 added 1 not.
}
