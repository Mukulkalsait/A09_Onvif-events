/* SOAP (Simple Object Access Protocol) is a protocol for exchanging structured information in web services.
* It uses XML to format messages and typically uses HTTP to transport them. */

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
/// Onvf Service Types  Enum:
pub enum OnvifService {
    Device,
    Media,
    Ptz,
    Events,
}

// Impl:
impl OnvifService {
    pub fn namespaces(&self) -> &'static str {
        match self {
            OnvifService::Device => "http://www.onvif.org/ver10/device/wsdl",
            OnvifService::Media => "http://www.onvif.org/ver10/media/wsdl",
            OnvifService::Ptz => "http://www.onvif.org/ver20/ptz/wsdl",
            OnvifService::Events => "http://www.onvif.org/ver10/events/wsdl",
        }
    }
    pub fn path(&self) -> &'static str {
        match self {
            OnvifService::Device => "/onvif/device_service",
            OnvifService::Media => "/onvif/media_service",
            OnvifService::Ptz => "/onvif/ptz_service",
            OnvifService::Events => "/onvif/events_service",
        }
    }
    pub fn action_prefix(&self) -> &'static str {
        match self {
            OnvifService::Device => "http://www.onvif.org/ver10/device/wsdl/",
            OnvifService::Media => "http://www.onvif.org/ver10/media/wsdl/",
            OnvifService::Ptz => "http://www.onvif.org/ver20/ptz/wsdl/",
            OnvifService::Events => "http://www.onvif.org/ver10/events/wsdl/",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Simple Object Access Protocol Request  Struct:
/// for onvif camera.
pub struct SoapRequest {
    /// # Service type: OnvifService {Device, Media, Ptz, Events}
    pub service: OnvifService,
    /// # Operation name.
    pub operation: String,
    /// # XMP Payload
    pub payload: String,
    /// # Some(WS-addressing Actuion)
    pub action: Option<String>,
    /// # Some(Message Id)
    pub message_id: Option<String>,
}

impl SoapRequest {
    /// # Create soap request
    pub fn new(service: OnvifService, operation: impl Into<String>) -> Self {
        let operation = operation.into();
        let action = Some(format!("{}{}", service.action_prefix(), operation));
        // Op: http://www.onvif.org/ver20/ptz/wsdl/ + operation

        Self {
            service,
            operation: operation.clone(),
            payload: format!("<{} xmlns=\"{}\"/>", operation, service.namespaces()),
            action,
            message_id: Some(format!("{}", uuid::Uuid::new_v4())),
        }
    }

    /// # set payload to costume & return
    pub fn with_payload(mut self, payload: impl Into<String>) -> Self {
        self.payload = payload.into();
        self
    }

    /// # Generate a complete SOAP involope from a prebuild SoapRequest.
    pub fn to_xml(&self) -> String {
        // IMP: to deref we use *, but here we used the value to consider as De_Ref as deref is expencive
        let action = self.action.as_deref().unwrap_or("");
        let message_id = self.message_id.as_deref().unwrap_or("");

        format!(
            r#"
<?xml version="1.0" encoding="UTF-8"?>
<s:Envelop xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://www.w3.2005/08/addressing">
    <s:Header>
        <a:Action s:mustUnderstand="1">{}</a:Action>
        <a:MessageID>{}</a:MessageID>
        <a:TO s:mustUnderstand="1">{}</a:TO >
    </s:Header>
    <s:Body>
        {}
    </s:Body>
<s:Envelop >
"#,
            action,
            message_id,
            self.service.path(),
            self.payload
        )
    }
}

/// Impl:
impl fmt::Display for SoapRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.to_xml()) }
}

#[derive(Debug, Default)]
/// SOAP buiolder Struct:
pub struct SoapRequestBuilder {
    service: Option<OnvifService>, // IMP: OnvifService
    operation: Option<String>,
    payload: Option<String>,
    action: Option<String>,
    message_id: Option<String>,
}

impl SoapRequestBuilder {
    /// gives default config ? None all?
    pub fn new() -> Self { Self::default() }
    /// update service function
    pub fn service(mut self, service: OnvifService) -> Self {
        self.service = Some(service);
        self
    }
    /// update opeoration function
    pub fn operation(mut self, opratioon: impl Into<String>) -> Self {
        self.operation = Some(opratioon.into());
        self
    }
    /// update paload function
    pub fn payload(mut self, payload: impl Into<String>) -> Self {
        self.payload = Some(payload.into());
        self
    }
    /// update action function
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
    pub fn build(self) -> Result<SoapRequest, String> {
        let service = self.service.ok_or("Service is required.")?;
        let operation = self.operation.ok_or("Operation is required.")?;

        let payload = self.payload.unwrap_or_else(|| format!("<{} xmlns=\"{}\"/>", operation, service.namespaces()));

        Ok(SoapRequest { service, operation, payload, action: self.action, message_id: self.message_id })
    }
}
