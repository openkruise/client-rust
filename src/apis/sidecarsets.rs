// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f -
// kopium version: 0.15.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "apps.kruise.io", version = "v1alpha1", kind = "SidecarSet", plural = "sidecarsets")]
#[kube(status = "SidecarSetStatus")]
#[kube(schema = "disabled")]
pub struct SidecarSetSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<SidecarSetContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<SidecarSetImagePullSecrets>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<SidecarSetInitContainers>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "injectionStrategy")]
    pub injection_strategy: Option<SidecarSetInjectionStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<SidecarSetSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updateStrategy")]
    pub update_strategy: Option<SidecarSetUpdateStrategy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetContainers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podInjectPolicy")]
    pub pod_inject_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shareVolumePolicy")]
    pub share_volume_policy: Option<SidecarSetContainersShareVolumePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transferEnv")]
    pub transfer_env: Option<Vec<SidecarSetContainersTransferEnv>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeStrategy")]
    pub upgrade_strategy: Option<SidecarSetContainersUpgradeStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetContainersShareVolumePolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetContainersTransferEnv {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envName")]
    pub env_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envNames")]
    pub env_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceContainerName")]
    pub source_container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceContainerNameFrom")]
    pub source_container_name_from: Option<SidecarSetContainersTransferEnvSourceContainerNameFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetContainersTransferEnvSourceContainerNameFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<SidecarSetContainersTransferEnvSourceContainerNameFromFieldRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetContainersTransferEnvSourceContainerNameFromFieldRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetContainersUpgradeStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hotUpgradeEmptyImage")]
    pub hot_upgrade_empty_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeType")]
    pub upgrade_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetImagePullSecrets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInitContainers {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podInjectPolicy")]
    pub pod_inject_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shareVolumePolicy")]
    pub share_volume_policy: Option<SidecarSetInitContainersShareVolumePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transferEnv")]
    pub transfer_env: Option<Vec<SidecarSetInitContainersTransferEnv>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeStrategy")]
    pub upgrade_strategy: Option<SidecarSetInitContainersUpgradeStrategy>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInitContainersShareVolumePolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInitContainersTransferEnv {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envName")]
    pub env_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "envNames")]
    pub env_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceContainerName")]
    pub source_container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sourceContainerNameFrom")]
    pub source_container_name_from: Option<SidecarSetInitContainersTransferEnvSourceContainerNameFrom>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInitContainersTransferEnvSourceContainerNameFrom {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<SidecarSetInitContainersTransferEnvSourceContainerNameFromFieldRef>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInitContainersTransferEnvSourceContainerNameFromFieldRef {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInitContainersUpgradeStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hotUpgradeEmptyImage")]
    pub hot_upgrade_empty_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upgradeType")]
    pub upgrade_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetInjectionStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SidecarSetSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetUpdateStrategy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxUnavailable")]
    pub max_unavailable: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<IntOrString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scatterStrategy")]
    pub scatter_strategy: Option<Vec<SidecarSetUpdateStrategyScatterStrategy>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<SidecarSetUpdateStrategySelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetUpdateStrategyScatterStrategy {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetUpdateStrategySelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<SidecarSetUpdateStrategySelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetUpdateStrategySelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SidecarSetStatus {
    #[serde(rename = "matchedPods")]
    pub matched_pods: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(rename = "readyPods")]
    pub ready_pods: i32,
    #[serde(rename = "updatedPods")]
    pub updated_pods: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "updatedReadyPods")]
    pub updated_ready_pods: Option<i32>,
}

