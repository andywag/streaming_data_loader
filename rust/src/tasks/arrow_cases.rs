use crate::{provider::provider_config::{ProviderConfig, SourceDescription, ProviderLength, HuggingDescription}, transport::{TransportConfig, TransportEnum}};


pub fn get_provider(source:HuggingDescription, test:bool) -> ProviderConfig {
    let source = SourceDescription::HuggingFace(source);

    if test {
        ProviderConfig {
            shuffle: None,
            flatten: None,
            length: ProviderLength::Iterations { iterations:1024 },
            source,
            filter: None,
        }
    }
    else {
        ProviderConfig {
            shuffle: Some(true),
            flatten: Some(true),
            length: ProviderLength::Epochs { epochs:3 },
            source,
            filter: None,
        }
    }
}

pub fn get_transport_config(test:bool) -> TransportConfig {
    if test {
        TransportConfig{transport:TransportEnum::Test}
    }
    else {
        TransportConfig{ transport: TransportEnum::Zmq { address: "ipc:///tmp/masking_train".to_string()} }
    }
}
