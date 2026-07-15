#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
pub use radroots_protocol_contract_v1::{
    CapabilityAvailabilityV1, CapabilityMaturityV1, MeshScopeIdV1, ProtocolEventClassV1,
    ProtocolSchemaMetadataV1, ReticulumDestinationV1, ReticulumTargetV1,
    TransportCapabilityDescriptorV1, TransportKindV1, validate_protocol_contract_v1,
};

pub const RUNTIME_CONTRACT_NAME_V1: &str = "radroots.runtime";
pub const RUNTIME_CONTRACT_VERSION_V1: u16 = 1;
pub const RUNTIME_OPERATION_SCHEMA_VERSION_V1: u16 = 1;
pub const REQUEST_DIGEST_ALGORITHM_V1: &str = "rfc8785_jcs_sha256";

macro_rules! runtime_operation_ids {
    ($( $variant:ident => $value:literal ),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum RuntimeOperationIdV1 {
            $( $variant, )+
        }

        impl RuntimeOperationIdV1 {
            pub fn as_str(self) -> &'static str {
                match self {
                    $( Self::$variant => $value, )+
                }
            }

            pub fn parse(value: &str) -> Result<Self, RuntimeContractErrorV1> {
                match value {
                    $( $value => Ok(Self::$variant), )+
                    _ => Err(RuntimeContractErrorV1::UnknownOperationId {
                        operation_id: value.to_string(),
                    }),
                }
            }

            pub fn request_schema_id(self) -> &'static str {
                match self {
                    $( Self::$variant => concat!("radroots.runtime.", $value, ".request.v1"), )+
                }
            }

            pub fn receipt_schema_id(self) -> &'static str {
                match self {
                    $( Self::$variant => concat!("radroots.runtime.", $value, ".receipt.v1"), )+
                }
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for RuntimeOperationIdV1 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for RuntimeOperationIdV1 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                Self::parse(value.as_str()).map_err(serde::de::Error::custom)
            }
        }
    };
}

runtime_operation_ids! {
    ProfileInspect => "profile.inspect",
    ProfileReset => "profile.reset",
    AccountCreate => "account.create",
    AccountImport => "account.import",
    AccountSelect => "account.select",
    AccountList => "account.list",
    AccountRemove => "account.remove",
    SignerStatus => "signer.status",
    StoreInspect => "store.inspect",
    StoreBackup => "store.backup",
    StoreRestore => "store.restore",
    FarmCreate => "farm.create",
    FarmUpdate => "farm.update",
    FarmPublish => "farm.publish",
    FarmGet => "farm.get",
    FarmList => "farm.list",
    ListingCreate => "listing.create",
    ListingUpdate => "listing.update",
    ListingPublish => "listing.publish",
    ListingPause => "listing.pause",
    ListingWithdraw => "listing.withdraw",
    ListingGet => "listing.get",
    ListingList => "listing.list",
    MarketPull => "market.pull",
    MarketSearch => "market.search",
    MarketGet => "market.get",
    BasketCreate => "basket.create",
    BasketGet => "basket.get",
    BasketList => "basket.list",
    BasketItemAdd => "basket.item.add",
    BasketItemUpdate => "basket.item.update",
    BasketItemRemove => "basket.item.remove",
    BasketQuote => "basket.quote",
    TradeRequest => "trade.request",
    TradeGet => "trade.get",
    TradeList => "trade.list",
    TradeAccept => "trade.accept",
    TradeDecline => "trade.decline",
    TradeCancel => "trade.cancel",
    ValidationStatus => "validation.status",
    ValidationReceiptGet => "validation.receipt.get",
    ValidationReceiptVerify => "validation.receipt.verify",
    SyncStatus => "sync.status",
    SyncPull => "sync.pull",
    SyncPush => "sync.push",
    HealthInspect => "health.inspect",
    TransportCapabilityList => "transport.capability.list",
    TransportConfigInspect => "transport.config.inspect",
    TransportConfigUpdate => "transport.config.update",
    TransportStatusInspect => "transport.status.inspect",
    TransportDeliveryInspect => "transport.delivery.inspect",
    TransportDeliveryRetry => "transport.delivery.retry",
    DiagnosticsInspect => "diagnostics.inspect",
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExecutionModeV1 {
    Embedded,
    Radrootsd,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationMutabilityV1 {
    Read,
    Mutation,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OperationRiskV1 {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApprovalRequirementV1 {
    None,
    ConditionalOrRequiredByMode,
    Required,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SignerRequirementV1 {
    None,
    Required,
    ConditionalRelayAuth,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdempotencyPolicyV1 {
    Forbidden,
    RequiredUuidV7,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DryRunSupportV1 {
    NotApplicable,
    PureLocalPlan,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeadlinePolicyV1 {
    DefaultBounded,
    OperationDeclared,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrivacyEffectV1 {
    None,
    PublicEvent,
    PrivateCoordination,
    PrivateStore,
    BackupRestore,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProjectionEffectV1 {
    None,
    ReadsProjection,
    WritesProjection,
    MayUpdateProjection,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransportCapabilityRouteV1 {
    pub local: bool,
    pub nostr: bool,
    pub reticulum: bool,
    pub deliver: bool,
    pub fetch: bool,
    pub synchronize: bool,
    pub diagnostics: bool,
}

impl TransportCapabilityRouteV1 {
    pub const fn none() -> Self {
        Self {
            local: false,
            nostr: false,
            reticulum: false,
            deliver: false,
            fetch: false,
            synchronize: false,
            diagnostics: false,
        }
    }

    pub const fn local() -> Self {
        Self {
            local: true,
            nostr: false,
            reticulum: false,
            deliver: false,
            fetch: false,
            synchronize: false,
            diagnostics: false,
        }
    }

    pub const fn delivery() -> Self {
        Self {
            local: false,
            nostr: true,
            reticulum: true,
            deliver: true,
            fetch: false,
            synchronize: false,
            diagnostics: false,
        }
    }

    pub const fn fetch() -> Self {
        Self {
            local: false,
            nostr: true,
            reticulum: true,
            deliver: false,
            fetch: true,
            synchronize: true,
            diagnostics: false,
        }
    }

    pub const fn diagnostics() -> Self {
        Self {
            local: true,
            nostr: true,
            reticulum: true,
            deliver: false,
            fetch: false,
            synchronize: false,
            diagnostics: true,
        }
    }

    pub fn includes_transport(self, kind: TransportKindV1) -> bool {
        match kind {
            TransportKindV1::Local => self.local,
            TransportKindV1::Nostr => self.nostr,
            TransportKindV1::Reticulum => self.reticulum,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeOperationDescriptorV1 {
    pub operation_id: RuntimeOperationIdV1,
    pub schema_version: u16,
    pub mutability: OperationMutabilityV1,
    pub risk: OperationRiskV1,
    pub approval: ApprovalRequirementV1,
    pub signer: SignerRequirementV1,
    pub transport_capability: TransportCapabilityRouteV1,
    pub idempotency: IdempotencyPolicyV1,
    pub dry_run: DryRunSupportV1,
    pub deadline: DeadlinePolicyV1,
    pub privacy: PrivacyEffectV1,
    pub projection: ProjectionEffectV1,
    pub maturity: CapabilityMaturityV1,
}

impl RuntimeOperationDescriptorV1 {
    pub fn request_schema_id(self) -> &'static str {
        self.operation_id.request_schema_id()
    }

    pub fn receipt_schema_id(self) -> &'static str {
        self.operation_id.receipt_schema_id()
    }
}

struct DescriptorSpecV1 {
    operation_id: RuntimeOperationIdV1,
    mutability: OperationMutabilityV1,
    risk: OperationRiskV1,
    approval: ApprovalRequirementV1,
    signer: SignerRequirementV1,
    transport_capability: TransportCapabilityRouteV1,
    idempotency: IdempotencyPolicyV1,
    dry_run: DryRunSupportV1,
    privacy: PrivacyEffectV1,
    projection: ProjectionEffectV1,
}

const fn read(
    operation_id: RuntimeOperationIdV1,
    risk: OperationRiskV1,
    transport_capability: TransportCapabilityRouteV1,
    privacy: PrivacyEffectV1,
    projection: ProjectionEffectV1,
) -> RuntimeOperationDescriptorV1 {
    descriptor(DescriptorSpecV1 {
        operation_id,
        mutability: OperationMutabilityV1::Read,
        risk,
        approval: ApprovalRequirementV1::None,
        signer: SignerRequirementV1::None,
        transport_capability,
        idempotency: IdempotencyPolicyV1::Forbidden,
        dry_run: DryRunSupportV1::NotApplicable,
        privacy,
        projection,
    })
}

const fn mutation(
    operation_id: RuntimeOperationIdV1,
    risk: OperationRiskV1,
    approval: ApprovalRequirementV1,
    signer: SignerRequirementV1,
    transport_capability: TransportCapabilityRouteV1,
    privacy: PrivacyEffectV1,
    projection: ProjectionEffectV1,
) -> RuntimeOperationDescriptorV1 {
    descriptor(DescriptorSpecV1 {
        operation_id,
        mutability: OperationMutabilityV1::Mutation,
        risk,
        approval,
        signer,
        transport_capability,
        idempotency: IdempotencyPolicyV1::RequiredUuidV7,
        dry_run: DryRunSupportV1::PureLocalPlan,
        privacy,
        projection,
    })
}

const fn descriptor(spec: DescriptorSpecV1) -> RuntimeOperationDescriptorV1 {
    RuntimeOperationDescriptorV1 {
        operation_id: spec.operation_id,
        schema_version: RUNTIME_OPERATION_SCHEMA_VERSION_V1,
        mutability: spec.mutability,
        risk: spec.risk,
        approval: spec.approval,
        signer: spec.signer,
        transport_capability: spec.transport_capability,
        idempotency: spec.idempotency,
        dry_run: spec.dry_run,
        deadline: DeadlinePolicyV1::DefaultBounded,
        privacy: spec.privacy,
        projection: spec.projection,
        maturity: CapabilityMaturityV1::Stable,
    }
}

pub const RUNTIME_OPERATION_DESCRIPTORS_V1: &[RuntimeOperationDescriptorV1] = &[
    read(
        RuntimeOperationIdV1::ProfileInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::ProfileReset,
        OperationRiskV1::Critical,
        ApprovalRequirementV1::Required,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::AccountCreate,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::AccountImport,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::AccountSelect,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    read(
        RuntimeOperationIdV1::AccountList,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::AccountRemove,
        OperationRiskV1::Critical,
        ApprovalRequirementV1::Required,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    read(
        RuntimeOperationIdV1::SignerStatus,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::None,
    ),
    read(
        RuntimeOperationIdV1::StoreInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::StoreBackup,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::BackupRestore,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::StoreRestore,
        OperationRiskV1::Critical,
        ApprovalRequirementV1::Required,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::BackupRestore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::FarmCreate,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::FarmUpdate,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::FarmPublish,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PublicEvent,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::FarmGet,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::FarmList,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::ListingCreate,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::ListingUpdate,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::ListingPublish,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PublicEvent,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    mutation(
        RuntimeOperationIdV1::ListingPause,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PublicEvent,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    mutation(
        RuntimeOperationIdV1::ListingWithdraw,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PublicEvent,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::ListingGet,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::ListingList,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::MarketPull,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::ConditionalRelayAuth,
        TransportCapabilityRouteV1::fetch(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::MarketSearch,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::MarketGet,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::BasketCreate,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    read(
        RuntimeOperationIdV1::BasketGet,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::BasketList,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::BasketItemAdd,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::BasketItemUpdate,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::BasketItemRemove,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::BasketQuote,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    mutation(
        RuntimeOperationIdV1::TradeRequest,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PrivateCoordination,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::TradeGet,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateCoordination,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::TradeList,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateCoordination,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::TradeAccept,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PrivateCoordination,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    mutation(
        RuntimeOperationIdV1::TradeDecline,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PrivateCoordination,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    mutation(
        RuntimeOperationIdV1::TradeCancel,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::Required,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PrivateCoordination,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::ValidationStatus,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::ValidationReceiptGet,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::ValidationReceiptVerify,
        OperationRiskV1::Medium,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::SyncStatus,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::diagnostics(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::SyncPull,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::ConditionalRelayAuth,
        TransportCapabilityRouteV1::fetch(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    mutation(
        RuntimeOperationIdV1::SyncPush,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::ConditionalRelayAuth,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PublicEvent,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::HealthInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::diagnostics(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::None,
    ),
    read(
        RuntimeOperationIdV1::TransportCapabilityList,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::diagnostics(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::None,
    ),
    read(
        RuntimeOperationIdV1::TransportConfigInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::TransportConfigUpdate,
        OperationRiskV1::High,
        ApprovalRequirementV1::ConditionalOrRequiredByMode,
        SignerRequirementV1::None,
        TransportCapabilityRouteV1::local(),
        PrivacyEffectV1::PrivateStore,
        ProjectionEffectV1::WritesProjection,
    ),
    read(
        RuntimeOperationIdV1::TransportStatusInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::diagnostics(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    read(
        RuntimeOperationIdV1::TransportDeliveryInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::diagnostics(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::ReadsProjection,
    ),
    mutation(
        RuntimeOperationIdV1::TransportDeliveryRetry,
        OperationRiskV1::Medium,
        ApprovalRequirementV1::None,
        SignerRequirementV1::ConditionalRelayAuth,
        TransportCapabilityRouteV1::delivery(),
        PrivacyEffectV1::PublicEvent,
        ProjectionEffectV1::MayUpdateProjection,
    ),
    read(
        RuntimeOperationIdV1::DiagnosticsInspect,
        OperationRiskV1::Low,
        TransportCapabilityRouteV1::diagnostics(),
        PrivacyEffectV1::None,
        ProjectionEffectV1::None,
    ),
];

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeRequestEnvelopeV1 {
    pub contract: String,
    pub contract_version: u16,
    pub operation_id: RuntimeOperationIdV1,
    pub operation_schema_version: u16,
    pub request_id: String,
    pub actor_pubkey: Option<String>,
    pub idempotency_key: Option<String>,
    pub approval: Option<ApprovalProofV1>,
    pub execution_mode: ExecutionModeV1,
    pub request_json: String,
}

impl RuntimeRequestEnvelopeV1 {
    pub fn validate(&self) -> Result<(), RuntimeContractErrorV1> {
        if self.contract != RUNTIME_CONTRACT_NAME_V1 {
            return Err(RuntimeContractErrorV1::InvalidContractName {
                contract: self.contract.clone(),
            });
        }
        if self.contract_version != RUNTIME_CONTRACT_VERSION_V1 {
            return Err(RuntimeContractErrorV1::UnsupportedContractVersion {
                version: self.contract_version,
            });
        }
        let descriptor = operation_descriptor(self.operation_id)?;
        if self.operation_schema_version != descriptor.schema_version {
            return Err(RuntimeContractErrorV1::UnsupportedOperationSchemaVersion {
                operation_id: self.operation_id,
                version: self.operation_schema_version,
            });
        }
        validate_idempotency(self.idempotency_key.as_deref(), descriptor)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeResponseEnvelopeV1 {
    pub contract: String,
    pub contract_version: u16,
    pub operation_id: RuntimeOperationIdV1,
    pub operation_schema_version: u16,
    pub request_id: String,
    pub request_digest: RequestDigestV1,
    pub receipt_json: Option<String>,
    pub error: Option<RadrootsErrorV1>,
}

impl RuntimeResponseEnvelopeV1 {
    pub fn validate(&self) -> Result<(), RuntimeContractErrorV1> {
        if self.contract != RUNTIME_CONTRACT_NAME_V1 {
            return Err(RuntimeContractErrorV1::InvalidContractName {
                contract: self.contract.clone(),
            });
        }
        if self.contract_version != RUNTIME_CONTRACT_VERSION_V1 {
            return Err(RuntimeContractErrorV1::UnsupportedContractVersion {
                version: self.contract_version,
            });
        }
        operation_descriptor(self.operation_id)?;
        self.request_digest.validate()?;
        match (self.receipt_json.is_some(), self.error.is_some()) {
            (true, false) | (false, true) => Ok(()),
            _ => Err(RuntimeContractErrorV1::InvalidResponseEnvelope),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequestDigestAlgorithmV1 {
    Rfc8785JcsSha256,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestDigestV1 {
    pub algorithm: RequestDigestAlgorithmV1,
    pub sha256_hex: String,
}

impl RequestDigestV1 {
    pub fn validate(&self) -> Result<(), RuntimeContractErrorV1> {
        if self.algorithm != RequestDigestAlgorithmV1::Rfc8785JcsSha256 {
            return Err(RuntimeContractErrorV1::InvalidRequestDigest);
        }
        if self.sha256_hex.len() != 64
            || !self
                .sha256_hex
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(RuntimeContractErrorV1::InvalidRequestDigest);
        }
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovalProofV1 {
    pub proof_id: String,
    pub operation_id: RuntimeOperationIdV1,
    pub request_digest: RequestDigestV1,
    pub signer_pubkey: String,
    pub signed_at_unix_ms: u64,
    pub expires_at_unix_ms: Option<u64>,
    pub signature: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeTransactionV1 {
    pub transaction_id: String,
    pub operation_id: RuntimeOperationIdV1,
    pub request_digest: RequestDigestV1,
    pub effects: RuntimeTransactionEffectsV1,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeTransactionEffectsV1 {
    pub operation_journal: bool,
    pub canonical_event_ingest: bool,
    pub inventory: bool,
    pub outbox: bool,
    pub delivery_plan: bool,
    pub receipt: bool,
    pub projection_generation: bool,
}

macro_rules! runtime_error_codes {
    ($( $variant:ident => $value:literal ),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum RuntimeErrorCodeV1 {
            $( $variant, )+
        }

        impl RuntimeErrorCodeV1 {
            pub fn as_str(self) -> &'static str {
                match self {
                    $( Self::$variant => $value, )+
                }
            }

            pub fn parse(value: &str) -> Result<Self, RuntimeContractErrorV1> {
                match value {
                    $( $value => Ok(Self::$variant), )+
                    _ => Err(RuntimeContractErrorV1::UnknownErrorCode {
                        code: value.to_string(),
                    }),
                }
            }
        }

        #[cfg(feature = "serde")]
        impl serde::Serialize for RuntimeErrorCodeV1 {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::Deserialize<'de> for RuntimeErrorCodeV1 {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = <String as serde::Deserialize>::deserialize(deserializer)?;
                Self::parse(value.as_str()).map_err(serde::de::Error::custom)
            }
        }
    };
}

runtime_error_codes! {
    InvalidArgument => "invalid_argument",
    UnsupportedContractVersion => "unsupported_contract_version",
    UnsupportedProfileSchema => "unsupported_profile_schema",
    SchemaTooNew => "schema_too_new",
    NotFound => "not_found",
    AmbiguousTrade => "ambiguous_trade",
    StaleListingRevision => "stale_listing_revision",
    PreconditionChanged => "precondition_changed",
    RevisionRequired => "revision_required",
    InventoryUnavailable => "inventory_unavailable",
    IdempotencyConflict => "idempotency_conflict",
    OperationInProgress => "operation_in_progress",
    ApprovalRequired => "approval_required",
    ApprovalInvalid => "approval_invalid",
    ApprovalExpired => "approval_expired",
    ApprovalReplayed => "approval_replayed",
    AuthorizationDenied => "authorization_denied",
    SignerCapabilityMissing => "signer_capability_missing",
    SignerUnavailable => "signer_unavailable",
    SignerRejected => "signer_rejected",
    SignerTimeout => "signer_timeout",
    SignerCancelled => "signer_cancelled",
    RelayAuthRequired => "relay_auth_required",
    RelayAuthRejected => "relay_auth_rejected",
    RelayPaymentRequired => "relay_payment_required",
    RelayPolicyRestricted => "relay_policy_restricted",
    RelayRateLimited => "relay_rate_limited",
    RelayPowRequired => "relay_pow_required",
    TransportPartial => "transport_partial",
    TransportOperationUnavailable => "transport_operation_unavailable",
    SyncSaturated => "sync_saturated",
    SyncPartial => "sync_partial",
    DeadlineExceeded => "deadline_exceeded",
    CancelledNoCommit => "cancelled_no_commit",
    LocalCommittedDeliveryPending => "local_committed_delivery_pending",
    DatabaseBusy => "database_busy",
    ProfileWriterInUse => "profile_writer_in_use",
    MaintenanceInProgress => "maintenance_in_progress",
    StorageIntegrityFailed => "storage_integrity_failed",
    StorageSpaceInsufficient => "storage_space_insufficient",
    ProjectionStale => "projection_stale",
    ProjectionFailed => "projection_failed",
    ProjectionGenerationChanged => "projection_generation_changed",
    InvalidCursor => "invalid_cursor",
    UnsupportedCapability => "unsupported_capability",
    DmRelayUnconfigured => "dm_relay_unconfigured",
    PrivateDataUnavailable => "private_data_unavailable",
    ValidationPending => "validation_pending",
    ValidationExpired => "validation_expired",
    ValidationReceiptConflict => "validation_receipt_conflict",
    ValidatorSetInvalid => "validator_set_invalid",
    MediaPolicyDenied => "media_policy_denied",
    BackupInvalid => "backup_invalid",
    BackupAuthenticationFailed => "backup_authentication_failed",
    RestoreFailed => "restore_failed",
    Backpressure => "backpressure",
    InternalError => "internal_error",
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RuntimeErrorClassV1 {
    Validation,
    Contract,
    Storage,
    Resource,
    Conflict,
    Operation,
    Authorization,
    Signer,
    Network,
    Sync,
    Runtime,
    Projection,
    Query,
    Capability,
    Privacy,
    Security,
    Maintenance,
    ValidationReceipt,
    Internal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeErrorDescriptorV1 {
    pub code: RuntimeErrorCodeV1,
    pub error_class: RuntimeErrorClassV1,
    pub retryable: bool,
}

pub const RUNTIME_ERROR_DESCRIPTORS_V1: &[RuntimeErrorDescriptorV1] = &[
    error(
        RuntimeErrorCodeV1::InvalidArgument,
        RuntimeErrorClassV1::Validation,
        false,
    ),
    error(
        RuntimeErrorCodeV1::UnsupportedContractVersion,
        RuntimeErrorClassV1::Contract,
        false,
    ),
    error(
        RuntimeErrorCodeV1::UnsupportedProfileSchema,
        RuntimeErrorClassV1::Storage,
        false,
    ),
    error(
        RuntimeErrorCodeV1::SchemaTooNew,
        RuntimeErrorClassV1::Storage,
        false,
    ),
    error(
        RuntimeErrorCodeV1::NotFound,
        RuntimeErrorClassV1::Resource,
        false,
    ),
    error(
        RuntimeErrorCodeV1::AmbiguousTrade,
        RuntimeErrorClassV1::Conflict,
        false,
    ),
    error(
        RuntimeErrorCodeV1::StaleListingRevision,
        RuntimeErrorClassV1::Conflict,
        false,
    ),
    error(
        RuntimeErrorCodeV1::PreconditionChanged,
        RuntimeErrorClassV1::Conflict,
        true,
    ),
    error(
        RuntimeErrorCodeV1::RevisionRequired,
        RuntimeErrorClassV1::Conflict,
        false,
    ),
    error(
        RuntimeErrorCodeV1::InventoryUnavailable,
        RuntimeErrorClassV1::Conflict,
        false,
    ),
    error(
        RuntimeErrorCodeV1::IdempotencyConflict,
        RuntimeErrorClassV1::Conflict,
        false,
    ),
    error(
        RuntimeErrorCodeV1::OperationInProgress,
        RuntimeErrorClassV1::Operation,
        true,
    ),
    error(
        RuntimeErrorCodeV1::ApprovalRequired,
        RuntimeErrorClassV1::Authorization,
        false,
    ),
    error(
        RuntimeErrorCodeV1::ApprovalInvalid,
        RuntimeErrorClassV1::Authorization,
        false,
    ),
    error(
        RuntimeErrorCodeV1::ApprovalExpired,
        RuntimeErrorClassV1::Authorization,
        false,
    ),
    error(
        RuntimeErrorCodeV1::ApprovalReplayed,
        RuntimeErrorClassV1::Authorization,
        false,
    ),
    error(
        RuntimeErrorCodeV1::AuthorizationDenied,
        RuntimeErrorClassV1::Authorization,
        false,
    ),
    error(
        RuntimeErrorCodeV1::SignerCapabilityMissing,
        RuntimeErrorClassV1::Signer,
        false,
    ),
    error(
        RuntimeErrorCodeV1::SignerUnavailable,
        RuntimeErrorClassV1::Signer,
        true,
    ),
    error(
        RuntimeErrorCodeV1::SignerRejected,
        RuntimeErrorClassV1::Signer,
        false,
    ),
    error(
        RuntimeErrorCodeV1::SignerTimeout,
        RuntimeErrorClassV1::Signer,
        true,
    ),
    error(
        RuntimeErrorCodeV1::SignerCancelled,
        RuntimeErrorClassV1::Signer,
        false,
    ),
    error(
        RuntimeErrorCodeV1::RelayAuthRequired,
        RuntimeErrorClassV1::Network,
        true,
    ),
    error(
        RuntimeErrorCodeV1::RelayAuthRejected,
        RuntimeErrorClassV1::Network,
        false,
    ),
    error(
        RuntimeErrorCodeV1::RelayPaymentRequired,
        RuntimeErrorClassV1::Network,
        false,
    ),
    error(
        RuntimeErrorCodeV1::RelayPolicyRestricted,
        RuntimeErrorClassV1::Network,
        false,
    ),
    error(
        RuntimeErrorCodeV1::RelayRateLimited,
        RuntimeErrorClassV1::Network,
        true,
    ),
    error(
        RuntimeErrorCodeV1::RelayPowRequired,
        RuntimeErrorClassV1::Network,
        false,
    ),
    error(
        RuntimeErrorCodeV1::TransportPartial,
        RuntimeErrorClassV1::Network,
        true,
    ),
    error(
        RuntimeErrorCodeV1::TransportOperationUnavailable,
        RuntimeErrorClassV1::Capability,
        false,
    ),
    error(
        RuntimeErrorCodeV1::SyncSaturated,
        RuntimeErrorClassV1::Sync,
        true,
    ),
    error(
        RuntimeErrorCodeV1::SyncPartial,
        RuntimeErrorClassV1::Sync,
        true,
    ),
    error(
        RuntimeErrorCodeV1::DeadlineExceeded,
        RuntimeErrorClassV1::Runtime,
        true,
    ),
    error(
        RuntimeErrorCodeV1::CancelledNoCommit,
        RuntimeErrorClassV1::Runtime,
        false,
    ),
    error(
        RuntimeErrorCodeV1::LocalCommittedDeliveryPending,
        RuntimeErrorClassV1::Operation,
        true,
    ),
    error(
        RuntimeErrorCodeV1::DatabaseBusy,
        RuntimeErrorClassV1::Storage,
        true,
    ),
    error(
        RuntimeErrorCodeV1::ProfileWriterInUse,
        RuntimeErrorClassV1::Storage,
        true,
    ),
    error(
        RuntimeErrorCodeV1::MaintenanceInProgress,
        RuntimeErrorClassV1::Storage,
        true,
    ),
    error(
        RuntimeErrorCodeV1::StorageIntegrityFailed,
        RuntimeErrorClassV1::Storage,
        false,
    ),
    error(
        RuntimeErrorCodeV1::StorageSpaceInsufficient,
        RuntimeErrorClassV1::Storage,
        true,
    ),
    error(
        RuntimeErrorCodeV1::ProjectionStale,
        RuntimeErrorClassV1::Projection,
        true,
    ),
    error(
        RuntimeErrorCodeV1::ProjectionFailed,
        RuntimeErrorClassV1::Projection,
        true,
    ),
    error(
        RuntimeErrorCodeV1::ProjectionGenerationChanged,
        RuntimeErrorClassV1::Projection,
        true,
    ),
    error(
        RuntimeErrorCodeV1::InvalidCursor,
        RuntimeErrorClassV1::Query,
        false,
    ),
    error(
        RuntimeErrorCodeV1::UnsupportedCapability,
        RuntimeErrorClassV1::Capability,
        false,
    ),
    error(
        RuntimeErrorCodeV1::DmRelayUnconfigured,
        RuntimeErrorClassV1::Privacy,
        false,
    ),
    error(
        RuntimeErrorCodeV1::PrivateDataUnavailable,
        RuntimeErrorClassV1::Privacy,
        false,
    ),
    error(
        RuntimeErrorCodeV1::ValidationPending,
        RuntimeErrorClassV1::ValidationReceipt,
        true,
    ),
    error(
        RuntimeErrorCodeV1::ValidationExpired,
        RuntimeErrorClassV1::ValidationReceipt,
        false,
    ),
    error(
        RuntimeErrorCodeV1::ValidationReceiptConflict,
        RuntimeErrorClassV1::ValidationReceipt,
        false,
    ),
    error(
        RuntimeErrorCodeV1::ValidatorSetInvalid,
        RuntimeErrorClassV1::ValidationReceipt,
        false,
    ),
    error(
        RuntimeErrorCodeV1::MediaPolicyDenied,
        RuntimeErrorClassV1::Security,
        false,
    ),
    error(
        RuntimeErrorCodeV1::BackupInvalid,
        RuntimeErrorClassV1::Maintenance,
        false,
    ),
    error(
        RuntimeErrorCodeV1::BackupAuthenticationFailed,
        RuntimeErrorClassV1::Maintenance,
        false,
    ),
    error(
        RuntimeErrorCodeV1::RestoreFailed,
        RuntimeErrorClassV1::Maintenance,
        true,
    ),
    error(
        RuntimeErrorCodeV1::Backpressure,
        RuntimeErrorClassV1::Runtime,
        true,
    ),
    error(
        RuntimeErrorCodeV1::InternalError,
        RuntimeErrorClassV1::Internal,
        false,
    ),
];

const fn error(
    code: RuntimeErrorCodeV1,
    error_class: RuntimeErrorClassV1,
    retryable: bool,
) -> RuntimeErrorDescriptorV1 {
    RuntimeErrorDescriptorV1 {
        code,
        error_class,
        retryable,
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadrootsErrorV1 {
    pub code: RuntimeErrorCodeV1,
    pub error_class: RuntimeErrorClassV1,
    pub retryable: bool,
    pub message: String,
    pub detail_json: Option<String>,
}

impl RadrootsErrorV1 {
    pub fn validate(&self) -> Result<(), RuntimeContractErrorV1> {
        let descriptor = error_descriptor(self.code)?;
        if descriptor.error_class != self.error_class || descriptor.retryable != self.retryable {
            return Err(RuntimeContractErrorV1::ErrorDescriptorMismatch { code: self.code });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuntimeContractErrorV1 {
    DuplicateOperationId {
        operation_id: RuntimeOperationIdV1,
    },
    DuplicateErrorCode {
        code: RuntimeErrorCodeV1,
    },
    MissingAmendedOperation {
        operation_id: RuntimeOperationIdV1,
    },
    UnknownOperationId {
        operation_id: String,
    },
    UnknownErrorCode {
        code: String,
    },
    UnsupportedContractVersion {
        version: u16,
    },
    UnsupportedOperationSchemaVersion {
        operation_id: RuntimeOperationIdV1,
        version: u16,
    },
    InvalidContractName {
        contract: String,
    },
    ReadIdempotencyForbidden {
        operation_id: RuntimeOperationIdV1,
    },
    MutationIdempotencyRequired {
        operation_id: RuntimeOperationIdV1,
    },
    InvalidUuidV7IdempotencyKey {
        operation_id: RuntimeOperationIdV1,
    },
    InvalidRequestDigest,
    InvalidResponseEnvelope,
    ErrorDescriptorMismatch {
        code: RuntimeErrorCodeV1,
    },
    ContractCatalogInvalid {
        message: String,
    },
}

impl core::fmt::Display for RuntimeContractErrorV1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DuplicateOperationId { operation_id } => {
                write!(f, "duplicate operation id {}", operation_id.as_str())
            }
            Self::DuplicateErrorCode { code } => {
                write!(f, "duplicate error code {}", code.as_str())
            }
            Self::MissingAmendedOperation { operation_id } => {
                write!(f, "missing amended operation {}", operation_id.as_str())
            }
            Self::UnknownOperationId { operation_id } => {
                write!(f, "unknown operation id {operation_id}")
            }
            Self::UnknownErrorCode { code } => write!(f, "unknown error code {code}"),
            Self::UnsupportedContractVersion { version } => {
                write!(f, "unsupported contract version {version}")
            }
            Self::UnsupportedOperationSchemaVersion {
                operation_id,
                version,
            } => write!(
                f,
                "unsupported operation schema version {version} for {}",
                operation_id.as_str()
            ),
            Self::InvalidContractName { contract } => write!(f, "invalid contract name {contract}"),
            Self::ReadIdempotencyForbidden { operation_id } => {
                write!(
                    f,
                    "read operation {} forbids idempotency",
                    operation_id.as_str()
                )
            }
            Self::MutationIdempotencyRequired { operation_id } => write!(
                f,
                "mutation operation {} requires UUIDv7 idempotency",
                operation_id.as_str()
            ),
            Self::InvalidUuidV7IdempotencyKey { operation_id } => write!(
                f,
                "operation {} has invalid UUIDv7 idempotency key",
                operation_id.as_str()
            ),
            Self::InvalidRequestDigest => f.write_str("invalid request digest"),
            Self::InvalidResponseEnvelope => f.write_str("invalid response envelope"),
            Self::ErrorDescriptorMismatch { code } => {
                write!(f, "error descriptor mismatch for {}", code.as_str())
            }
            Self::ContractCatalogInvalid { message } => f.write_str(message),
        }
    }
}

pub fn operation_descriptor(
    operation_id: RuntimeOperationIdV1,
) -> Result<RuntimeOperationDescriptorV1, RuntimeContractErrorV1> {
    RUNTIME_OPERATION_DESCRIPTORS_V1
        .iter()
        .copied()
        .find(|descriptor| descriptor.operation_id == operation_id)
        .ok_or_else(|| RuntimeContractErrorV1::UnknownOperationId {
            operation_id: operation_id.as_str().to_string(),
        })
}

pub fn error_descriptor(
    code: RuntimeErrorCodeV1,
) -> Result<RuntimeErrorDescriptorV1, RuntimeContractErrorV1> {
    RUNTIME_ERROR_DESCRIPTORS_V1
        .iter()
        .copied()
        .find(|descriptor| descriptor.code == code)
        .ok_or_else(|| RuntimeContractErrorV1::UnknownErrorCode {
            code: code.as_str().to_string(),
        })
}

pub fn validate_runtime_contract_v1() -> Result<(), RuntimeContractErrorV1> {
    validate_protocol_contract_v1().map_err(|error| {
        RuntimeContractErrorV1::ContractCatalogInvalid {
            message: error.to_string(),
        }
    })?;
    validate_operation_descriptors()?;
    validate_error_descriptors()
}

fn validate_operation_descriptors() -> Result<(), RuntimeContractErrorV1> {
    let mut operation_ids = BTreeSet::new();
    for descriptor in RUNTIME_OPERATION_DESCRIPTORS_V1 {
        if !operation_ids.insert(descriptor.operation_id) {
            return Err(RuntimeContractErrorV1::DuplicateOperationId {
                operation_id: descriptor.operation_id,
            });
        }
        match (descriptor.mutability, descriptor.idempotency) {
            (OperationMutabilityV1::Read, IdempotencyPolicyV1::Forbidden)
            | (OperationMutabilityV1::Mutation, IdempotencyPolicyV1::RequiredUuidV7) => {}
            _ => {
                return Err(RuntimeContractErrorV1::ContractCatalogInvalid {
                    message: format!(
                        "operation {} has invalid idempotency policy",
                        descriptor.operation_id.as_str()
                    ),
                });
            }
        }
        if descriptor.schema_version != RUNTIME_OPERATION_SCHEMA_VERSION_V1 {
            return Err(RuntimeContractErrorV1::UnsupportedOperationSchemaVersion {
                operation_id: descriptor.operation_id,
                version: descriptor.schema_version,
            });
        }
    }
    for amended in [
        RuntimeOperationIdV1::TransportCapabilityList,
        RuntimeOperationIdV1::TransportConfigInspect,
        RuntimeOperationIdV1::TransportConfigUpdate,
        RuntimeOperationIdV1::TransportStatusInspect,
        RuntimeOperationIdV1::TransportDeliveryInspect,
        RuntimeOperationIdV1::TransportDeliveryRetry,
        RuntimeOperationIdV1::SyncStatus,
        RuntimeOperationIdV1::SyncPull,
        RuntimeOperationIdV1::SyncPush,
        RuntimeOperationIdV1::DiagnosticsInspect,
    ] {
        if !operation_ids.contains(&amended) {
            return Err(RuntimeContractErrorV1::MissingAmendedOperation {
                operation_id: amended,
            });
        }
    }
    for delivery in [
        RuntimeOperationIdV1::FarmPublish,
        RuntimeOperationIdV1::ListingPublish,
        RuntimeOperationIdV1::ListingPause,
        RuntimeOperationIdV1::ListingWithdraw,
        RuntimeOperationIdV1::TradeRequest,
        RuntimeOperationIdV1::TradeAccept,
        RuntimeOperationIdV1::TradeDecline,
        RuntimeOperationIdV1::TradeCancel,
        RuntimeOperationIdV1::SyncPush,
        RuntimeOperationIdV1::TransportDeliveryRetry,
    ] {
        let descriptor = operation_descriptor(delivery)?;
        if !descriptor.transport_capability.deliver
            || !descriptor
                .transport_capability
                .includes_transport(TransportKindV1::Nostr)
            || !descriptor
                .transport_capability
                .includes_transport(TransportKindV1::Reticulum)
        {
            return Err(RuntimeContractErrorV1::ContractCatalogInvalid {
                message: format!(
                    "operation {} must use Nostr and Reticulum delivery capability",
                    descriptor.operation_id.as_str()
                ),
            });
        }
    }
    Ok(())
}

fn validate_error_descriptors() -> Result<(), RuntimeContractErrorV1> {
    let mut codes = BTreeSet::new();
    for descriptor in RUNTIME_ERROR_DESCRIPTORS_V1 {
        if !codes.insert(descriptor.code) {
            return Err(RuntimeContractErrorV1::DuplicateErrorCode {
                code: descriptor.code,
            });
        }
    }
    if !codes.contains(&RuntimeErrorCodeV1::TransportOperationUnavailable) {
        return Err(RuntimeContractErrorV1::UnknownErrorCode {
            code: "transport_operation_unavailable".to_string(),
        });
    }
    Ok(())
}

fn validate_idempotency(
    idempotency_key: Option<&str>,
    descriptor: RuntimeOperationDescriptorV1,
) -> Result<(), RuntimeContractErrorV1> {
    match descriptor.idempotency {
        IdempotencyPolicyV1::Forbidden => {
            if idempotency_key.is_some() {
                return Err(RuntimeContractErrorV1::ReadIdempotencyForbidden {
                    operation_id: descriptor.operation_id,
                });
            }
            Ok(())
        }
        IdempotencyPolicyV1::RequiredUuidV7 => {
            let Some(key) = idempotency_key else {
                return Err(RuntimeContractErrorV1::MutationIdempotencyRequired {
                    operation_id: descriptor.operation_id,
                });
            };
            if !is_uuid_v7(key) {
                return Err(RuntimeContractErrorV1::InvalidUuidV7IdempotencyKey {
                    operation_id: descriptor.operation_id,
                });
            }
            Ok(())
        }
    }
}

fn is_uuid_v7(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes[8] == b'-'
        && bytes[13] == b'-'
        && bytes[18] == b'-'
        && bytes[23] == b'-'
        && bytes[14] == b'7'
        && matches!(bytes[19], b'8' | b'9' | b'a' | b'b')
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 8 | 13 | 18 | 23) || byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    const UUID_V7: &str = "01890f78-9abc-7def-8abc-123456789abc";

    #[test]
    fn runtime_contract_catalogs_validate() {
        validate_runtime_contract_v1().expect("runtime contract validates");
    }

    #[test]
    fn reads_reject_idempotency_keys() {
        let request = RuntimeRequestEnvelopeV1 {
            contract: RUNTIME_CONTRACT_NAME_V1.to_string(),
            contract_version: RUNTIME_CONTRACT_VERSION_V1,
            operation_id: RuntimeOperationIdV1::ProfileInspect,
            operation_schema_version: RUNTIME_OPERATION_SCHEMA_VERSION_V1,
            request_id: "request-1".to_string(),
            actor_pubkey: None,
            idempotency_key: Some(UUID_V7.to_string()),
            approval: None,
            execution_mode: ExecutionModeV1::Embedded,
            request_json: "{}".to_string(),
        };

        assert!(matches!(
            request.validate(),
            Err(RuntimeContractErrorV1::ReadIdempotencyForbidden { .. })
        ));
    }

    #[test]
    fn mutations_require_uuid_v7_idempotency_keys() {
        let mut request = RuntimeRequestEnvelopeV1 {
            contract: RUNTIME_CONTRACT_NAME_V1.to_string(),
            contract_version: RUNTIME_CONTRACT_VERSION_V1,
            operation_id: RuntimeOperationIdV1::ListingCreate,
            operation_schema_version: RUNTIME_OPERATION_SCHEMA_VERSION_V1,
            request_id: "request-2".to_string(),
            actor_pubkey: None,
            idempotency_key: None,
            approval: None,
            execution_mode: ExecutionModeV1::Embedded,
            request_json: "{}".to_string(),
        };

        assert!(matches!(
            request.validate(),
            Err(RuntimeContractErrorV1::MutationIdempotencyRequired { .. })
        ));

        request.idempotency_key = Some("listing-create".to_string());
        assert!(matches!(
            request.validate(),
            Err(RuntimeContractErrorV1::InvalidUuidV7IdempotencyKey { .. })
        ));

        request.idempotency_key = Some(UUID_V7.to_string());
        request.validate().expect("valid UUIDv7 idempotency");
    }

    #[test]
    fn delivery_operations_are_transport_capability_descriptors() {
        for operation_id in [
            RuntimeOperationIdV1::FarmPublish,
            RuntimeOperationIdV1::ListingPublish,
            RuntimeOperationIdV1::TradeRequest,
            RuntimeOperationIdV1::SyncPush,
        ] {
            let descriptor = operation_descriptor(operation_id).expect("descriptor");
            assert!(descriptor.transport_capability.deliver);
            assert!(
                descriptor
                    .transport_capability
                    .includes_transport(TransportKindV1::Nostr)
            );
            assert!(
                descriptor
                    .transport_capability
                    .includes_transport(TransportKindV1::Reticulum)
            );
        }
    }

    #[test]
    fn execution_mode_is_not_transport_identity() {
        assert_ne!(
            ExecutionModeV1::Radrootsd as u8,
            TransportKindV1::Reticulum as u8
        );
        assert!(TransportKindV1::parse("proxy").is_err());
    }

    #[test]
    fn operation_ids_reject_retired_preview_names() {
        for value in [
            ["sync.try_reticulum", "_preview_now"].concat(),
            ["transport.reticulum", "_preview.status"].concat(),
            ["transport.", "hybrid", ".publish"].concat(),
            ["radrootsd.", "proxy", ".publish"].concat(),
        ] {
            assert!(RuntimeOperationIdV1::parse(value.as_str()).is_err());
        }
    }
}
