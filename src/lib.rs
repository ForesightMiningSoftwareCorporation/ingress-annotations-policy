use lazy_static::lazy_static;

use guest::prelude::*;
use kubewarden_policy_sdk::wapc_guest as guest;

use k8s_openapi::api::networking::v1 as apinetworking;

extern crate kubewarden_policy_sdk as kubewarden;
use kubewarden::{logging, protocol_version_guest, request::ValidationRequest, validate_settings};

mod settings;
use settings::Settings;

use slog::{info, o, warn, Logger};

lazy_static! {
    static ref LOG_DRAIN: Logger = Logger::root(
        logging::KubewardenDrain::new(),
        o!("policy" => "sample-policy")
    );
}

#[no_mangle]
pub extern "C" fn wapc_init() {
    register_function("validate", validate);
    register_function("validate_settings", validate_settings::<Settings>);
    register_function("protocol_version", protocol_version_guest);
}

fn validate(payload: &[u8]) -> CallResult {
    let validation_request: ValidationRequest<Settings> = ValidationRequest::new(payload)?;
    let settings = validation_request.settings;
    info!(LOG_DRAIN, "starting validation");
    // TODO: you can unmarshal any Kubernetes API type you are interested in
    match serde_json::from_value::<apinetworking::Ingress>(validation_request.request.object) {
        Ok(mut ingress) => {
            let mut mutated: bool = false;
            let mut annotations = ingress.metadata.annotations.clone().unwrap_or_default();
            settings.annotations.into_iter().for_each(|(k, v)| {
                annotations.entry(k).or_insert_with(|| {
                    mutated = true;
                    v
                });
            });
            if mutated {
                info!(LOG_DRAIN, "mutating resource");
                ingress.metadata.annotations = Some(annotations);
                let ingress_spec = match serde_json::to_value(ingress) {
                    Ok(v) => v,
                    Err(msg) => {
                        return kubewarden::reject_request(Some(msg.to_string()), None, None, None);
                    }
                };
                kubewarden::mutate_request(ingress_spec)
            } else {
                info!(LOG_DRAIN, "accepting resource");
                kubewarden::accept_request()
            }
        }
        Err(_) => {
            warn!(LOG_DRAIN, "cannot unmarshal resource: this policy does not know how to evaluate this resource; accept it");
            kubewarden::accept_request()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kubewarden_policy_sdk::test::Testcase;

    #[test]
    fn mutate_pod_with_default_annotations() -> Result<(), ()> {
        let request_file = "test_data/ingress_creation.json";
        let tc = Testcase {
            name: String::from("Valid name"),
            fixture_file: String::from(request_file),
            expected_validation_result: true,
            settings: Settings::default(),
        };

        let res = tc.eval(validate).unwrap();
        assert!(
            res.mutated_object.is_some(),
            "Something mutated with test case: {}",
            tc.name,
        );

        Ok(())
    }

    #[test]
    fn dont_overwrite_pod_with_existing_annotations() -> Result<(), ()> {
        let request_file = "test_data/ingress_creation_existing_annotations.json";
        let tc = Testcase {
            name: String::from("Valid name"),
            fixture_file: String::from(request_file),
            expected_validation_result: true,
            settings: Settings::default(),
        };

        let res = tc.eval(validate).unwrap();
        assert!(
            res.mutated_object.is_none(),
            "Something mutated with test case: {}",
            tc.name,
        );

        Ok(())
    }
}
