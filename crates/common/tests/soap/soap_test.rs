use common::camera::CameraID;
use common::onvif_error::{self, OnvifError};
use common::soap::{OnvifService, SoapRequest};
use std::fmt::Write;
use test_case::test_case;

#[test_case(OnvifService::Device, "GetDeviceInformation")]
#[test_case(OnvifService::Media, "GetMediaInformation")]
#[test_case(OnvifService::Ptz, "GetPtzInformation")]
#[test_case(OnvifService::Events, "GetEventsInformation")]
fn test_soap_request_simple(onvif_service: OnvifService, operation: &'static str) {
    let req = SoapRequest::new(onvif_service, operation);
    let xml = req.to_xml();
    let action = onvif_service.action_prefix();

    assert!(xml.contains("<s:Envelop"));
    assert!(xml.contains("<s:Body>"));
    assert!(xml.contains(operation));
    assert!(xml.contains(action));
}

#[test_case(OnvifService::Device, "GetProfiles")]
#[test_case(OnvifService::Media, "GetProfiles")]
#[test_case(OnvifService::Ptz, "GetProfiles")]
#[test_case(OnvifService::Events, "GetProfiles")]
fn test_soap_request_with_payload(onvif_service: OnvifService, operation: &'static str) {
    let action_prefix = onvif_service.action_prefix();
    let mut payload_builder = String::new();
    write!(&mut payload_builder, "r#\"<{} xmlns={}/>\"#", operation, action_prefix).unwrap();
    let payload = payload_builder.as_str();

    let request = SoapRequest::new(onvif_service, operation).with_payload(payload);
    let xml = request.to_xml();

    assert!(xml.contains(operation));
    assert!(xml.contains(action_prefix))
}

#[test_case(OnvifService::Device, "GetProfiles")]
#[test_case(OnvifService::Media, "GetProfiles")]
#[test_case(OnvifService::Ptz, "GetProfiles")]
#[test_case(OnvifService::Events, "GetProfiles")]
fn test_soap_request_builder(onvif_service: OnvifService, operation: &'static str) {
    // let request = SoapRequestBuilder::new().service(onvif_service).operations(operation).build().unwrap();
    // let x = "asdf".to_string();
}
