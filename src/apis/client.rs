use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    catalog_api: Box<crate::apis::CatalogApi>,
    service_bindings_api: Box<crate::apis::ServiceBindingsApi>,
    service_instances_api: Box<crate::apis::ServiceInstancesApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            catalog_api: Box::new(crate::apis::CatalogApiClient::new(rc.clone())),
            service_bindings_api: Box::new(crate::apis::ServiceBindingsApiClient::new(rc.clone())),
            service_instances_api: Box::new(crate::apis::ServiceInstancesApiClient::new(
                rc.clone(),
            )),
        }
    }

    pub fn catalog_api(&self) -> &crate::apis::CatalogApi {
        self.catalog_api.as_ref()
    }

    pub fn service_bindings_api(&self) -> &crate::apis::ServiceBindingsApi {
        self.service_bindings_api.as_ref()
    }

    pub fn service_instances_api(&self) -> &crate::apis::ServiceInstancesApi {
        self.service_instances_api.as_ref()
    }
}
