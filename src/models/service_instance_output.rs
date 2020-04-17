use rocl::models::ServiceInstanceResource;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInstanceOutput {
    #[serde(
        rename = "service_instance_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_instance_id: Option<String>,
    #[serde(
        rename = "service_instance_resource",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_instance_resource: Option<ServiceInstanceResource>,
}

impl ServiceInstanceOutput {
    pub fn new() -> ServiceInstanceOutput {
        ServiceInstanceOutput {
            service_instance_id: None,
            service_instance_resource: None,
        }
    }
}
