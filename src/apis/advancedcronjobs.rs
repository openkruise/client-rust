// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af -
// kopium version: 0.15.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

/// AdvancedCronJobSpec defines the desired state of AdvancedCronJob
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "apps.kruise.io", version = "v1alpha1", kind = "AdvancedCronJob", plural = "advancedcronjobs")]
#[kube(namespaced)]
#[kube(status = "AdvancedCronJobStatus")]
pub struct AdvancedCronJobSpec {
    /// Specifies how to treat concurrent executions of a Job. Valid values are: - "Allow" (default): allows CronJobs to run concurrently; - "Forbid": forbids concurrent runs, skipping next run if previous run hasn't finished yet; - "Replace": cancels currently running job and replaces it with a new one
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrencyPolicy")]
    pub concurrency_policy: Option<AdvancedCronJobConcurrencyPolicy>,
    /// The number of failed finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedJobsHistoryLimit")]
    pub failed_jobs_history_limit: Option<i32>,
    /// Paused will pause the cron job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// The schedule in Cron format, see https://en.wikipedia.org/wiki/Cron.
    pub schedule: String,
    /// Optional deadline in seconds for starting the job if it misses scheduled time for any reason.  Missed jobs executions will be counted as failed ones.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startingDeadlineSeconds")]
    pub starting_deadline_seconds: Option<i64>,
    /// The number of successful finished jobs to retain. This is a pointer to distinguish between explicit zero and not specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successfulJobsHistoryLimit")]
    pub successful_jobs_history_limit: Option<i32>,
    /// Specifies the job that will be created when executing a CronJob.
    pub template: AdvancedCronJobTemplate,
    /// The time zone name for the given schedule, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones. If not specified, this will default to the time zone of the kruise-controller-manager process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeZone")]
    pub time_zone: Option<String>,
}

/// AdvancedCronJobSpec defines the desired state of AdvancedCronJob
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum AdvancedCronJobConcurrencyPolicy {
    Allow,
    Forbid,
    Replace,
}

/// Specifies the job that will be created when executing a CronJob.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobTemplate {
    /// Specifies the broadcastjob that will be created when executing a BroadcastCronJob.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "broadcastJobTemplate")]
    pub broadcast_job_template: Option<AdvancedCronJobTemplateBroadcastJobTemplate>,
    /// Specifies the job that will be created when executing a CronJob.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobTemplate")]
    pub job_template: Option<HashMap<String, serde_json::Value>>,
}

/// Specifies the broadcastjob that will be created when executing a BroadcastCronJob.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobTemplateBroadcastJobTemplate {
    /// Standard object's metadata of the jobs created from this template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AdvancedCronJobTemplateBroadcastJobTemplateMetadata>,
    /// Specification of the desired behavior of the broadcastjob.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<AdvancedCronJobTemplateBroadcastJobTemplateSpec>,
}

/// Standard object's metadata of the jobs created from this template.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobTemplateBroadcastJobTemplateMetadata {
}

/// Specification of the desired behavior of the broadcastjob.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobTemplateBroadcastJobTemplateSpec {
    /// CompletionPolicy indicates the completion policy of the job. Default is Always CompletionPolicyType.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionPolicy")]
    pub completion_policy: Option<AdvancedCronJobTemplateBroadcastJobTemplateSpecCompletionPolicy>,
    /// FailurePolicy indicates the behavior of the job, when failed pod is found.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failurePolicy")]
    pub failure_policy: Option<AdvancedCronJobTemplateBroadcastJobTemplateSpecFailurePolicy>,
    /// Parallelism specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when the work left to do is less than max parallelism. Not setting this value means no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<IntOrString>,
    /// Paused will pause the job.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Template describes the pod that will be created when executing a job.
    pub template: HashMap<String, serde_json::Value>,
}

/// CompletionPolicy indicates the completion policy of the job. Default is Always CompletionPolicyType.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobTemplateBroadcastJobTemplateSpecCompletionPolicy {
    /// ActiveDeadlineSeconds specifies the duration in seconds relative to the startTime that the job may be active before the system tries to terminate it; value must be positive integer. Only works for Always type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "activeDeadlineSeconds")]
    pub active_deadline_seconds: Option<i64>,
    /// ttlSecondsAfterFinished limits the lifetime of a Job that has finished execution (either Complete or Failed). If this field is set, ttlSecondsAfterFinished after the Job finishes, it is eligible to be automatically deleted. When the Job is being deleted, its lifecycle guarantees (e.g. finalizers) will be honored. If this field is unset, the Job won't be automatically deleted. If this field is set to zero, the Job becomes eligible to be deleted immediately after it finishes. This field is alpha-level and is only honored by servers that enable the TTLAfterFinished feature. Only works for Always type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ttlSecondsAfterFinished")]
    pub ttl_seconds_after_finished: Option<i32>,
    /// Type indicates the type of the CompletionPolicy. Default is Always.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// FailurePolicy indicates the behavior of the job, when failed pod is found.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobTemplateBroadcastJobTemplateSpecFailurePolicy {
    /// RestartLimit specifies the number of retries before marking the pod failed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "restartLimit")]
    pub restart_limit: Option<i32>,
    /// Type indicates the type of FailurePolicyType. Default is FailurePolicyTypeFailFast.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// AdvancedCronJobStatus defines the observed state of AdvancedCronJob
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobStatus {
    /// A list of pointers to currently running jobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<Vec<AdvancedCronJobStatusActive>>,
    /// Information when was the last time the job was successfully scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScheduleTime")]
    pub last_schedule_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ObjectReference contains enough information to let you inspect or modify the referred object. --- New uses of this type are discouraged because of difficulty describing its usage when embedded in APIs. 1. Ignored fields.  It includes many fields which are not generally honored.  For instance, ResourceVersion and FieldPath are both very rarely valid in actual usage. 2. Invalid usage help.  It is impossible to add specific help for individual usage.  In most embedded usages, there are particular restrictions like, "must refer only to types A and B" or "UID not honored" or "name must be restricted". Those cannot be well described when embedded. 3. Inconsistent validation.  Because the usages are different, the validation rules are different by usage, which makes it hard for users to predict what will happen. 4. The fields are both imprecise and overly precise.  Kind is not a precise mapping to a URL. This can produce ambiguity during interpretation and require a REST mapping.  In most cases, the dependency is on the group,resource tuple and the version of the actual struct is irrelevant. 5. We cannot easily change it.  Because this type is embedded in many locations, updates to this type will affect numerous schemas.  Don't make new APIs embed an underspecified API type they do not control. 
///  Instead of using this type, create a locally provided and used type that is well-focused on your reference. For example, ServiceReferences for admission registration: https://github.com/kubernetes/api/blob/release-1.17/admissionregistration/v1/types.go#L533 .
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct AdvancedCronJobStatusActive {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
