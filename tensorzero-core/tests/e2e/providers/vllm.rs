use std::collections::HashMap;

use crate::providers::common::{E2ETestProvider, E2ETestProviders};

crate::generate_provider_tests!(get_providers);
crate::generate_batch_inference_tests!(get_providers);

async fn get_providers() -> E2ETestProviders {
    let credentials = match std::env::var("VLLM_API_KEY") {
        Ok(key) => HashMap::from([("vllm_api_key".to_string(), key)]),
        Err(_) => HashMap::new(),
    };

    let providers = vec![E2ETestProvider {
        supports_batch_inference: false,
        variant_name: "vllm".to_string(),
        model_name: "qwen2.5-0.5b-instruct-vllm".into(),
        model_provider_name: "vllm".into(),
        credentials: HashMap::new(),
    }];

    let extra_body_providers = vec![E2ETestProvider {
        supports_batch_inference: false,
        variant_name: "vllm-extra-body".to_string(),
        model_name: "qwen2.5-0.5b-instruct-vllm".into(),
        model_provider_name: "vllm".into(),
        credentials: HashMap::new(),
    }];

    let bad_auth_extra_headers = vec![E2ETestProvider {
        supports_batch_inference: false,
        variant_name: "vllm-extra-headers".to_string(),
        model_name: "qwen2.5-0.5b-instruct-vllm".into(),
        model_provider_name: "vllm".into(),
        credentials: HashMap::new(),
    }];

    let json_providers = vec![
        E2ETestProvider {
            supports_batch_inference: false,
            variant_name: "vllm".to_string(),
            model_name: "qwen2.5-0.5b-instruct-vllm".into(),
            model_provider_name: "vllm".into(),
            credentials: HashMap::new(),
        },
        E2ETestProvider {
            supports_batch_inference: false,
            variant_name: "vllm-strict".to_string(),
            model_name: "qwen2.5-0.5b-instruct-vllm".into(),
            model_provider_name: "vllm".into(),
            credentials: HashMap::new(),
        },
    ];

    let inference_params_dynamic_providers = vec![E2ETestProvider {
        supports_batch_inference: false,
        variant_name: "vllm-dynamic".to_string(),
        model_name: "qwen2.5-0.5b-instruct-vllm-dynamic".into(),
        model_provider_name: "vllm".into(),
        credentials,
    }];

    let tool_providers = vec![E2ETestProvider {
        supports_batch_inference: false,
        variant_name: "vllm".to_string(),
        model_name: "qwen2.5-0.5b-instruct-vllm".to_string(),
        model_provider_name: "vllm".to_string(),
        credentials: HashMap::new(),
    }];

    E2ETestProviders {
        simple_inference: providers.clone(),
        extra_body_inference: extra_body_providers,
        bad_auth_extra_headers,
        reasoning_inference: vec![],
        inference_params_inference: providers.clone(),
        inference_params_dynamic_credentials: inference_params_dynamic_providers,
        tool_use_inference: tool_providers.clone(),
        tool_multi_turn_inference: tool_providers.clone(),
        dynamic_tool_use_inference: tool_providers.clone(),
        parallel_tool_use_inference: tool_providers.clone(),
        json_mode_inference: json_providers.clone(),
        json_mode_off_inference: vec![],
        image_inference: vec![],
        pdf_inference: vec![],
        shorthand_inference: vec![],
    }
}
