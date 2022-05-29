use std::convert::TryFrom;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Hash, EnumIter)]
#[allow(non_camel_case_types)]
pub enum KotlinClassName {
    String,
    Date,
    ArrayList,
    Map,
    HashMap,
    Pair,
    Core,
    Storage_Result,
    Storage_Result_Ok,
    Storage_Result_Err,
    EnvError,
    Action_Ctx,
    Action_Load,
    Action_StreamingServer,
    Action_Unload,
    ActionCtx,
    ActionCtx_Authenticate,
    ActionCtx_Logout,
    ActionCtx_UpdateSettings,
    ActionCtx_AddToLibrary,
    ActionCtx_RemoveFromLibrary,
    ActionCtx_RewindLibraryItem,
    ActionCtx_PushUserToAPI,
    ActionCtx_PullUserFromAPI,
    ActionCtx_PushAddonsToAPI,
    ActionCtx_PullAddonsFromAPI,
    ActionCtx_SyncLibraryWithAPI,
    Action_Link,
    ActionLink,
    ActionLink_ReadData,
    ActionLoad,
    ActionLoad_Link,
    ActionLoad_CatalogWithFilters,
    ActionLoad_CatalogsWithExtra,
    ActionLoad_LibraryWithFilters,
    ActionLoad_MetaDetails,
    ActionStreamingServer,
    ActionStreamingServer_Reload,
    ActionStreamingServer_UpdateSettings,
    RuntimeEvent,
    RuntimeEvent_NewState,
    RuntimeEvent_CoreEvent,
    MetaItem,
    MetaItemBehaviorHints,
    PosterShape,
    ResourceRequest,
    ResourcePath,
    CatalogsWithExtra_Selected,
    StreamingServer_Selected,
    StreamingServer_Settings,
    Profile_Settings,
    GDPRConsent,
    AuthRequest,
    AuthRequest_Login,
    AuthRequest_LoginWithToken,
    AuthRequest_Register,
    AuthRequest_GDPRConsentRequest,
    MetaDetails_Selected,
    CatalogWithFilters_Selected,
    LibraryWithFilters_Selected,
    LibraryWithFilters_LibraryRequest,
    LibraryWithFilters_Sort,
    Event,
    Event_ProfilePushedToStorage,
    Event_LibraryItemsPushedToStorage,
    Event_UserPulledFromAPI,
    Event_UserPushedToAPI,
    Event_AddonsPulledFromAPI,
    Event_AddonsPushedToAPI,
    Event_LibrarySyncWithAPIPlanned,
    Event_LibraryItemsPushedToAPI,
    Event_LibraryItemsPulledFromAPI,
    Event_UserAuthenticated,
    Event_UserLoggedOut,
    Event_SessionDeleted,
    Event_AddonInstalled,
    Event_AddonUpgraded,
    Event_AddonUninstalled,
    Event_SettingsUpdated,
    Event_LibraryItemAdded,
    Event_LibraryItemRemoved,
    Event_LibraryItemRewinded,
    Event_Error,
}

impl TryFrom<String> for KotlinClassName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        KotlinClassName::iter()
            .find(|class_name| class_name.value() == value)
            .ok_or(format!("Class name not found: {}", value))
    }
}

impl KotlinClassName {
    pub fn value(&self) -> &str {
        match self {
            KotlinClassName::String => "java/lang/String",
            KotlinClassName::Date => "java/util/Date",
            KotlinClassName::ArrayList => "java/util/ArrayList",
            KotlinClassName::Map => "java/util/Map",
            KotlinClassName::HashMap => "java/util/HashMap",
            KotlinClassName::Pair => "kotlin/Pair",
            KotlinClassName::Core => "com/stremio/core/Core",
            KotlinClassName::Storage_Result => "com/stremio/core/Storage$Result",
            KotlinClassName::Storage_Result_Ok => "com/stremio/core/Storage$Result$Ok",
            KotlinClassName::Storage_Result_Err => "com/stremio/core/Storage$Result$Err",
            KotlinClassName::EnvError => "com/stremio/core/runtime/EnvError",
            KotlinClassName::Action_Ctx => "com/stremio/core/runtime/msg/Action$Ctx",
            KotlinClassName::Action_Load => "com/stremio/core/runtime/msg/Action$Load",
            KotlinClassName::Action_StreamingServer => {
                "com/stremio/core/runtime/msg/Action$StreamingServer"
            }
            KotlinClassName::Action_Unload => "com/stremio/core/runtime/msg/Action$Unload",
            KotlinClassName::ActionCtx => "com/stremio/core/runtime/msg/ActionCtx",
            KotlinClassName::ActionCtx_Authenticate => {
                "com/stremio/core/runtime/msg/ActionCtx$Authenticate"
            }
            KotlinClassName::ActionCtx_Logout => "com/stremio/core/runtime/msg/ActionCtx$Logout",
            KotlinClassName::ActionCtx_UpdateSettings => {
                "com/stremio/core/runtime/msg/ActionCtx$UpdateSettings"
            }
            KotlinClassName::ActionCtx_AddToLibrary => {
                "com/stremio/core/runtime/msg/ActionCtx$AddToLibrary"
            }
            KotlinClassName::ActionCtx_RemoveFromLibrary => {
                "com/stremio/core/runtime/msg/ActionCtx$RemoveFromLibrary"
            }
            KotlinClassName::ActionCtx_RewindLibraryItem => {
                "com/stremio/core/runtime/msg/ActionCtx$RewindLibraryItem"
            }
            KotlinClassName::ActionCtx_PushUserToAPI => {
                "com/stremio/core/runtime/msg/ActionCtx$PushUserToAPI"
            }
            KotlinClassName::ActionCtx_PullUserFromAPI => {
                "com/stremio/core/runtime/msg/ActionCtx$PullUserFromAPI"
            }
            KotlinClassName::ActionCtx_PushAddonsToAPI => {
                "com/stremio/core/runtime/msg/ActionCtx$PushAddonsToAPI"
            }
            KotlinClassName::ActionCtx_PullAddonsFromAPI => {
                "com/stremio/core/runtime/msg/ActionCtx$PullAddonsFromAPI"
            }
            KotlinClassName::ActionCtx_SyncLibraryWithAPI => {
                "com/stremio/core/runtime/msg/ActionCtx$SyncLibraryWithAPI"
            }
            KotlinClassName::Action_Link => "com/stremio/core/runtime/msg/Action$Link",
            KotlinClassName::ActionLink => "com/stremio/core/runtime/msg/ActionLink",
            KotlinClassName::ActionLink_ReadData => {
                "com/stremio/core/runtime/msg/ActionLink$ReadData"
            }
            KotlinClassName::ActionLoad => "com/stremio/core/runtime/msg/ActionLoad",
            KotlinClassName::ActionLoad_Link => "com/stremio/core/runtime/msg/ActionLoad$Link",
            KotlinClassName::ActionLoad_CatalogWithFilters => {
                "com/stremio/core/runtime/msg/ActionLoad$CatalogWithFilters"
            }
            KotlinClassName::ActionLoad_CatalogsWithExtra => {
                "com/stremio/core/runtime/msg/ActionLoad$CatalogsWithExtra"
            }
            KotlinClassName::ActionLoad_LibraryWithFilters => {
                "com/stremio/core/runtime/msg/ActionLoad$LibraryWithFilters"
            }
            KotlinClassName::ActionLoad_MetaDetails => {
                "com/stremio/core/runtime/msg/ActionLoad$MetaDetails"
            }
            KotlinClassName::ActionStreamingServer => {
                "com/stremio/core/runtime/msg/ActionStreamingServer"
            }
            KotlinClassName::ActionStreamingServer_Reload => {
                "com/stremio/core/runtime/msg/ActionStreamingServer$Reload"
            }
            KotlinClassName::ActionStreamingServer_UpdateSettings => {
                "com/stremio/core/runtime/msg/ActionStreamingServer$UpdateSettings"
            }
            KotlinClassName::RuntimeEvent => "com/stremio/core/runtime/RuntimeEvent",
            KotlinClassName::RuntimeEvent_NewState => {
                "com/stremio/core/runtime/RuntimeEvent$NewState"
            }
            KotlinClassName::RuntimeEvent_CoreEvent => {
                "com/stremio/core/runtime/RuntimeEvent$CoreEvent"
            }
            KotlinClassName::MetaItem => "com/stremio/core/types/resource/MetaItem",
            KotlinClassName::MetaItemBehaviorHints => {
                "com/stremio/core/types/resource/MetaItemBehaviorHints"
            }
            KotlinClassName::PosterShape => "com/stremio/core/types/resource/PosterShape",
            KotlinClassName::ResourceRequest => "com/stremio/core/types/addon/ResourceRequest",
            KotlinClassName::ResourcePath => "com/stremio/core/types/addon/ResourcePath",
            KotlinClassName::CatalogsWithExtra_Selected => {
                "com/stremio/core/models/CatalogsWithExtra$Selected"
            }
            KotlinClassName::StreamingServer_Selected => {
                "com/stremio/core/models/StreamingServer$Selected"
            }
            KotlinClassName::StreamingServer_Settings => {
                "com/stremio/core/models/StreamingServer$Settings"
            }
            KotlinClassName::AuthRequest => "com/stremio/core/types/api/AuthRequest",
            KotlinClassName::AuthRequest_Login => "com/stremio/core/types/api/AuthRequest$Login",
            KotlinClassName::AuthRequest_LoginWithToken => {
                "com/stremio/core/types/api/AuthRequest$LoginWithToken"
            }
            KotlinClassName::AuthRequest_Register => {
                "com/stremio/core/types/api/AuthRequest$Register"
            }
            KotlinClassName::AuthRequest_GDPRConsentRequest => {
                "com/stremio/core/types/api/AuthRequest$GDPRConsentRequest"
            }
            KotlinClassName::Profile_Settings => "com/stremio/core/types/profile/Profile$Settings",
            KotlinClassName::GDPRConsent => "com/stremio/core/types/profile/GDPRConsent",
            KotlinClassName::MetaDetails_Selected => "com/stremio/core/models/MetaDetails$Selected",
            KotlinClassName::CatalogWithFilters_Selected => {
                "com/stremio/core/models/CatalogWithFilters$Selected"
            }
            KotlinClassName::LibraryWithFilters_Selected => {
                "com/stremio/core/models/LibraryWithFilters$Selected"
            }
            KotlinClassName::LibraryWithFilters_LibraryRequest => {
                "com/stremio/core/models/LibraryWithFilters$LibraryRequest"
            }
            KotlinClassName::LibraryWithFilters_Sort => {
                "com/stremio/core/models/LibraryWithFilters$Sort"
            }
            KotlinClassName::Event => "com/stremio/core/runtime/msg/Event",
            KotlinClassName::Event_ProfilePushedToStorage => {
                "com/stremio/core/runtime/msg/Event$ProfilePushedToStorage"
            }
            KotlinClassName::Event_LibraryItemsPushedToStorage => {
                "com/stremio/core/runtime/msg/Event$LibraryItemsPushedToStorage"
            }
            KotlinClassName::Event_UserPulledFromAPI => {
                "com/stremio/core/runtime/msg/Event$UserPulledFromAPI"
            }
            KotlinClassName::Event_UserPushedToAPI => {
                "com/stremio/core/runtime/msg/Event$UserPushedToAPI"
            }
            KotlinClassName::Event_AddonsPulledFromAPI => {
                "com/stremio/core/runtime/msg/Event$AddonsPulledFromAPI"
            }
            KotlinClassName::Event_AddonsPushedToAPI => {
                "com/stremio/core/runtime/msg/Event$AddonsPushedToAPI"
            }
            KotlinClassName::Event_LibrarySyncWithAPIPlanned => {
                "com/stremio/core/runtime/msg/Event$LibrarySyncWithAPIPlanned"
            }
            KotlinClassName::Event_LibraryItemsPushedToAPI => {
                "com/stremio/core/runtime/msg/Event$LibraryItemsPushedToAPI"
            }
            KotlinClassName::Event_LibraryItemsPulledFromAPI => {
                "com/stremio/core/runtime/msg/Event$LibraryItemsPulledFromAPI"
            }
            KotlinClassName::Event_UserAuthenticated => {
                "com/stremio/core/runtime/msg/Event$UserAuthenticated"
            }
            KotlinClassName::Event_UserLoggedOut => {
                "com/stremio/core/runtime/msg/Event$UserLoggedOut"
            }
            KotlinClassName::Event_SessionDeleted => {
                "com/stremio/core/runtime/msg/Event$SessionDeleted"
            }
            KotlinClassName::Event_AddonInstalled => {
                "com/stremio/core/runtime/msg/Event$AddonInstalled"
            }
            KotlinClassName::Event_AddonUpgraded => {
                "com/stremio/core/runtime/msg/Event$AddonUpgraded"
            }
            KotlinClassName::Event_AddonUninstalled => {
                "com/stremio/core/runtime/msg/Event$AddonUninstalled"
            }
            KotlinClassName::Event_SettingsUpdated => {
                "com/stremio/core/runtime/msg/Event$SettingsUpdated"
            }
            KotlinClassName::Event_LibraryItemAdded => {
                "com/stremio/core/runtime/msg/Event$LibraryItemAdded"
            }
            KotlinClassName::Event_LibraryItemRemoved => {
                "com/stremio/core/runtime/msg/Event$LibraryItemRemoved"
            }
            KotlinClassName::Event_LibraryItemRewinded => {
                "com/stremio/core/runtime/msg/Event$LibraryItemRewinded"
            }
            KotlinClassName::Event_Error => "com/stremio/core/runtime/msg/Event$Error",
        }
    }
}
