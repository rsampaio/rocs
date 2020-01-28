/*
 * Open Service Broker API
 *
 * The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.
 *
 * The version of the OpenAPI document: master - might contain changes that are not yet released
 * Contact: open-service-broker-api@googlegroups.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "requires", skip_serializing_if = "Option::is_none")]
    pub requires: Option<Requires>,
    #[serde(rename = "bindable")]
    pub bindable: bool,
    /// See [Service Metadata Conventions](https://github.com/openservicebrokerapi/servicebroker/blob/master/profile.md#service-metadata) for more details.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "dashboard_client", skip_serializing_if = "Option::is_none")]
    pub dashboard_client: Option<crate::models::DashboardClient>,
    #[serde(rename = "plan_updateable", skip_serializing_if = "Option::is_none")]
    pub plan_updateable: Option<bool>,
    #[serde(rename = "plans")]
    pub plans: Vec<crate::models::Plan>,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<crate::models::Extension>>,
}

impl Service {
    pub fn new(
        name: String,
        id: String,
        description: String,
        bindable: bool,
        plans: Vec<crate::models::Plan>,
    ) -> Service {
        Service {
            name: name,
            id: id,
            description: description,
            tags: None,
            requires: None,
            bindable: bindable,
            metadata: None,
            dashboard_client: None,
            plan_updateable: None,
            plans: plans,
            extensions: None,
        }
    }
}

///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Requires {
    #[serde(rename = "syslog_drain")]
    SyslogDrain,
    #[serde(rename = "route_forwarding")]
    RouteForwarding,
    #[serde(rename = "volume_mount")]
    VolumeMount,
}
