#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "linkedReadProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_read_properties: Vec<String>,
    #[serde(rename = "linkedWriteProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_write_properties: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    #[serde(rename = "AnnotationName", default, skip_serializing_if = "Option::is_none")]
    pub annotation_name: Option<String>,
    #[serde(rename = "Category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "EventTime", default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    #[serde(rename = "RelatedAnnotation", default, skip_serializing_if = "Option::is_none")]
    pub related_annotation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnotationError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnotationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Annotation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponent {
    #[serde(flatten)]
    pub components_resource: ComponentsResource,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentApiKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "linkedReadProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_read_properties: Vec<String>,
    #[serde(rename = "linkedWriteProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_write_properties: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentApiKeyListResult {
    pub value: Vec<ApplicationInsightsComponentApiKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentAnalyticsItem {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Scope", default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<application_insights_component_analytics_item::Scope>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<application_insights_component_analytics_item::Type>,
    #[serde(rename = "TimeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "TimeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationInsightsComponentAnalyticsItemProperties>,
}
pub mod application_insights_component_analytics_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scope {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "user")]
        User,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "query")]
        Query,
        #[serde(rename = "recent")]
        Recent,
        #[serde(rename = "function")]
        Function,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentAnalyticsItemProperties {
    #[serde(rename = "functionAlias", default, skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentAvailableFeatures {
    #[serde(rename = "Result", default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<ApplicationInsightsComponentFeature>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentBillingFeatures {
    #[serde(rename = "DataVolumeCap", default, skip_serializing_if = "Option::is_none")]
    pub data_volume_cap: Option<ApplicationInsightsComponentDataVolumeCap>,
    #[serde(rename = "CurrentBillingFeatures", default, skip_serializing_if = "Vec::is_empty")]
    pub current_billing_features: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentDataVolumeCap {
    #[serde(rename = "Cap", default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<f64>,
    #[serde(rename = "ResetTime", default, skip_serializing_if = "Option::is_none")]
    pub reset_time: Option<i64>,
    #[serde(rename = "WarningThreshold", default, skip_serializing_if = "Option::is_none")]
    pub warning_threshold: Option<i64>,
    #[serde(rename = "StopSendNotificationWhenHitThreshold", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_threshold: Option<bool>,
    #[serde(rename = "StopSendNotificationWhenHitCap", default, skip_serializing_if = "Option::is_none")]
    pub stop_send_notification_when_hit_cap: Option<bool>,
    #[serde(rename = "MaxHistoryCap", default, skip_serializing_if = "Option::is_none")]
    pub max_history_cap: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentExportConfiguration {
    #[serde(rename = "ExportId", default, skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "InstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
    #[serde(rename = "RecordTypes", default, skip_serializing_if = "Option::is_none")]
    pub record_types: Option<String>,
    #[serde(rename = "ApplicationName", default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "SubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "ResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "DestinationStorageSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_subscription_id: Option<String>,
    #[serde(rename = "DestinationStorageLocationId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_location_id: Option<String>,
    #[serde(rename = "DestinationAccountId", default, skip_serializing_if = "Option::is_none")]
    pub destination_account_id: Option<String>,
    #[serde(rename = "DestinationType", default, skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "IsUserEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_user_enabled: Option<String>,
    #[serde(rename = "LastUserUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_user_update: Option<String>,
    #[serde(rename = "NotificationQueueEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notification_queue_enabled: Option<String>,
    #[serde(rename = "ExportStatus", default, skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "LastSuccessTime", default, skip_serializing_if = "Option::is_none")]
    pub last_success_time: Option<String>,
    #[serde(rename = "LastGapTime", default, skip_serializing_if = "Option::is_none")]
    pub last_gap_time: Option<String>,
    #[serde(rename = "PermanentErrorReason", default, skip_serializing_if = "Option::is_none")]
    pub permanent_error_reason: Option<String>,
    #[serde(rename = "StorageName", default, skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
    #[serde(rename = "ContainerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}
pub type ApplicationInsightsComponentExportConfigurationListResult = Vec<ApplicationInsightsComponentExportConfiguration>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentExportRequest {
    #[serde(rename = "RecordTypes", default, skip_serializing_if = "Option::is_none")]
    pub record_types: Option<String>,
    #[serde(rename = "DestinationType", default, skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "DestinationAddress", default, skip_serializing_if = "Option::is_none")]
    pub destination_address: Option<String>,
    #[serde(rename = "IsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<String>,
    #[serde(rename = "NotificationQueueEnabled", default, skip_serializing_if = "Option::is_none")]
    pub notification_queue_enabled: Option<String>,
    #[serde(rename = "NotificationQueueUri", default, skip_serializing_if = "Option::is_none")]
    pub notification_queue_uri: Option<String>,
    #[serde(rename = "DestinationStorageSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_subscription_id: Option<String>,
    #[serde(rename = "DestinationStorageLocationId", default, skip_serializing_if = "Option::is_none")]
    pub destination_storage_location_id: Option<String>,
    #[serde(rename = "DestinationAccountId", default, skip_serializing_if = "Option::is_none")]
    pub destination_account_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentFavorite {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Config", default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "FavoriteId", default, skip_serializing_if = "Option::is_none")]
    pub favorite_id: Option<String>,
    #[serde(rename = "FavoriteType", default, skip_serializing_if = "Option::is_none")]
    pub favorite_type: Option<application_insights_component_favorite::FavoriteType>,
    #[serde(rename = "SourceType", default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "TimeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    #[serde(rename = "Tags", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "Category", default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "IsGeneratedFromTemplate", default, skip_serializing_if = "Option::is_none")]
    pub is_generated_from_template: Option<bool>,
    #[serde(rename = "UserId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
pub mod application_insights_component_favorite {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FavoriteType {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "user")]
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentFeature {
    #[serde(rename = "FeatureName", default, skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "MeterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "MeterRateFrequency", default, skip_serializing_if = "Option::is_none")]
    pub meter_rate_frequency: Option<String>,
    #[serde(rename = "ResouceId", default, skip_serializing_if = "Option::is_none")]
    pub resouce_id: Option<String>,
    #[serde(rename = "IsHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "Capabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ApplicationInsightsComponentFeatureCapability>,
    #[serde(rename = "Title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "IsMainFeature", default, skip_serializing_if = "Option::is_none")]
    pub is_main_feature: Option<bool>,
    #[serde(rename = "SupportedAddonFeatures", default, skip_serializing_if = "Option::is_none")]
    pub supported_addon_features: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentFeatureCapabilities {
    #[serde(rename = "SupportExportData", default, skip_serializing_if = "Option::is_none")]
    pub support_export_data: Option<bool>,
    #[serde(rename = "BurstThrottlePolicy", default, skip_serializing_if = "Option::is_none")]
    pub burst_throttle_policy: Option<String>,
    #[serde(rename = "MetadataClass", default, skip_serializing_if = "Option::is_none")]
    pub metadata_class: Option<String>,
    #[serde(rename = "LiveStreamMetrics", default, skip_serializing_if = "Option::is_none")]
    pub live_stream_metrics: Option<bool>,
    #[serde(rename = "ApplicationMap", default, skip_serializing_if = "Option::is_none")]
    pub application_map: Option<bool>,
    #[serde(rename = "WorkItemIntegration", default, skip_serializing_if = "Option::is_none")]
    pub work_item_integration: Option<bool>,
    #[serde(rename = "PowerBIIntegration", default, skip_serializing_if = "Option::is_none")]
    pub power_bi_integration: Option<bool>,
    #[serde(rename = "OpenSchema", default, skip_serializing_if = "Option::is_none")]
    pub open_schema: Option<bool>,
    #[serde(rename = "ProactiveDetection", default, skip_serializing_if = "Option::is_none")]
    pub proactive_detection: Option<bool>,
    #[serde(rename = "AnalyticsIntegration", default, skip_serializing_if = "Option::is_none")]
    pub analytics_integration: Option<bool>,
    #[serde(rename = "MultipleStepWebTest", default, skip_serializing_if = "Option::is_none")]
    pub multiple_step_web_test: Option<bool>,
    #[serde(rename = "ApiAccessLevel", default, skip_serializing_if = "Option::is_none")]
    pub api_access_level: Option<String>,
    #[serde(rename = "TrackingType", default, skip_serializing_if = "Option::is_none")]
    pub tracking_type: Option<String>,
    #[serde(rename = "DailyCap", default, skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<f64>,
    #[serde(rename = "DailyCapResetTime", default, skip_serializing_if = "Option::is_none")]
    pub daily_cap_reset_time: Option<f64>,
    #[serde(rename = "ThrottleRate", default, skip_serializing_if = "Option::is_none")]
    pub throttle_rate: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentFeatureCapability {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "Unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "MeterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "MeterRateFrequency", default, skip_serializing_if = "Option::is_none")]
    pub meter_rate_frequency: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentListResult {
    pub value: Vec<ApplicationInsightsComponent>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentProactiveDetectionConfiguration {
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "SendEmailsToSubscriptionOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_emails_to_subscription_owners: Option<bool>,
    #[serde(rename = "CustomEmails", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_emails: Vec<String>,
    #[serde(rename = "LastUpdatedTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "RuleDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub rule_definitions: Option<application_insights_component_proactive_detection_configuration::RuleDefinitions>,
}
pub mod application_insights_component_proactive_detection_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RuleDefinitions {
        #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "DisplayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "HelpUrl", default, skip_serializing_if = "Option::is_none")]
        pub help_url: Option<String>,
        #[serde(rename = "IsHidden", default, skip_serializing_if = "Option::is_none")]
        pub is_hidden: Option<bool>,
        #[serde(rename = "IsEnabledByDefault", default, skip_serializing_if = "Option::is_none")]
        pub is_enabled_by_default: Option<bool>,
        #[serde(rename = "IsInPreview", default, skip_serializing_if = "Option::is_none")]
        pub is_in_preview: Option<bool>,
        #[serde(rename = "SupportsEmailNotifications", default, skip_serializing_if = "Option::is_none")]
        pub supports_email_notifications: Option<bool>,
    }
}
pub type ApplicationInsightsComponentProactiveDetectionConfigurationListResult =
    Vec<ApplicationInsightsComponentProactiveDetectionConfiguration>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentProperties {
    #[serde(rename = "ApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AppId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "Application_Type")]
    pub application_type: application_insights_component_properties::ApplicationType,
    #[serde(rename = "Flow_Type", default, skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<application_insights_component_properties::FlowType>,
    #[serde(rename = "Request_Source", default, skip_serializing_if = "Option::is_none")]
    pub request_source: Option<application_insights_component_properties::RequestSource>,
    #[serde(rename = "InstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub instrumentation_key: Option<String>,
    #[serde(rename = "CreationDate", default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "TenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "HockeyAppId", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_id: Option<String>,
    #[serde(rename = "HockeyAppToken", default, skip_serializing_if = "Option::is_none")]
    pub hockey_app_token: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "SamplingPercentage", default, skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<f64>,
    #[serde(rename = "ConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[serde(rename = "RetentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i64>,
    #[serde(rename = "DisableIpMasking", default, skip_serializing_if = "Option::is_none")]
    pub disable_ip_masking: Option<bool>,
    #[serde(rename = "ImmediatePurgeDataOn30Days", default, skip_serializing_if = "Option::is_none")]
    pub immediate_purge_data_on30_days: Option<bool>,
    #[serde(rename = "PrivateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<PrivateLinkScopedResource>,
    #[serde(rename = "IngestionMode", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_mode: Option<application_insights_component_properties::IngestionMode>,
}
pub mod application_insights_component_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ApplicationType {
        #[serde(rename = "web")]
        Web,
        #[serde(rename = "other")]
        Other,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FlowType {
        Bluefield,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RequestSource {
        #[serde(rename = "rest")]
        Rest,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IngestionMode {
        ApplicationInsights,
        ApplicationInsightsWithDiagnosticSettings,
        LogAnalytics,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentQuotaStatus {
    #[serde(rename = "AppId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "ShouldBeThrottled", default, skip_serializing_if = "Option::is_none")]
    pub should_be_throttled: Option<bool>,
    #[serde(rename = "ExpirationTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsComponentWebTestLocation {
    #[serde(rename = "DisplayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInsightsWebTestLocationsListResult {
    pub value: Vec<ApplicationInsightsComponentWebTestLocation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeBody {
    pub table: String,
    pub filters: Vec<ComponentPurgeBodyFilters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeBodyFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentPurgeStatusResponse {
    pub status: component_purge_status_response::Status,
}
pub mod component_purge_status_response {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "completed")]
        Completed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComponentsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorFieldContract {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnosticcontext: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkProperties {
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "targetId", default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyWorkbook {
    #[serde(flatten)]
    pub my_workbook_resource: MyWorkbookResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<my_workbook::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MyWorkbookProperties>,
}
pub mod my_workbook {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "shared")]
        Shared,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyWorkbookError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorFieldContract>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyWorkbookProperties {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "serializedData")]
    pub serialized_data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "timeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    pub category: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyWorkbookResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyWorkbooksListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MyWorkbook>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopedResource {
    #[serde(rename = "ResourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ScopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTest {
    #[serde(flatten)]
    pub webtests_resource: WebtestsResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<web_test::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebTestProperties>,
}
pub mod web_test {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestGeolocation {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestProperties {
    #[serde(rename = "SyntheticMonitorId")]
    pub synthetic_monitor_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Frequency", default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    #[serde(rename = "Timeout", default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename = "Kind")]
    pub kind: web_test_properties::Kind,
    #[serde(rename = "RetryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub retry_enabled: Option<bool>,
    #[serde(rename = "Locations")]
    pub locations: Vec<WebTestGeolocation>,
    #[serde(rename = "Configuration", default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<web_test_properties::Configuration>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
pub mod web_test_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "ping")]
        Ping,
        #[serde(rename = "multistep")]
        Multistep,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Configuration {
        #[serde(rename = "WebTest", default, skip_serializing_if = "Option::is_none")]
        pub web_test: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebtestsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemConfiguration {
    #[serde(rename = "ConnectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "ConfigDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub config_display_name: Option<String>,
    #[serde(rename = "IsDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ConfigProperties", default, skip_serializing_if = "Option::is_none")]
    pub config_properties: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemConfigurationError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemConfigurationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkItemConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkItemCreateConfiguration {
    #[serde(rename = "ConnectorId", default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    #[serde(rename = "ConnectorDataConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub connector_data_configuration: Option<String>,
    #[serde(rename = "ValidateOnly", default, skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(rename = "WorkItemProperties", default, skip_serializing_if = "Option::is_none")]
    pub work_item_properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    #[serde(flatten)]
    pub workbook_resource: WorkbookResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<workbook::Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkbookProperties>,
}
pub mod workbook {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "shared")]
        Shared,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorFieldContract>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookProperties {
    pub name: String,
    #[serde(rename = "serializedData")]
    pub serialized_data: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "workbookId")]
    pub workbook_id: String,
    pub kind: workbook_properties::Kind,
    #[serde(rename = "timeModified", default, skip_serializing_if = "Option::is_none")]
    pub time_modified: Option<String>,
    pub category: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
}
pub mod workbook_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "shared")]
        Shared,
        #[serde(rename = "user")]
        User,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbookResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbooksListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workbook>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebTestListResult {
    pub value: Vec<WebTest>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
