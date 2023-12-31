// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -f -
// kopium version: 0.15.0

use kube::CustomResource;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug)]
#[kube(group = "apps.kruise.io", version = "v1alpha1", kind = "EphemeralJob", plural = "ephemeraljobs")]
#[kube(namespaced)]
#[kube(status = "EphemeralJobStatus")]
#[kube(schema = "disabled")]
pub struct EphemeralJobSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDeadlineSeconds")]
    pub active_deadline_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    pub selector: EphemeralJobSelector,
    pub template: EphemeralJobTemplate,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecondsAfterFinished")]
    pub ttl_seconds_after_finished: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EphemeralJobSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<EphemeralJobSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EphemeralJobSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EphemeralJobTemplate {
    #[serde(rename = "ephemeralContainers")]
    pub ephemeral_containers: HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EphemeralJobStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<EphemeralJobStatusConditions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EphemeralJobStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastProbeTime")]
    pub last_probe_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

