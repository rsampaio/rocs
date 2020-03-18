use models::service_binding_resource::ServiceBindingResource;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBindingOutput {
    #[serde(rename = "service_binding_id", skip_serializing_if = "Option::is_none")]
    pub service_binding_id: Option<String>,
    #[serde(
        rename = "service_binding_resource",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_binding_resource: Option<ServiceBindingResource>,
}

impl ServiceBindingOutput {
    pub fn new() -> ServiceBindingOutput {
        ServiceBindingOutput {
            service_binding_id: None,
            service_binding_resource: None,
        }
    }
}
