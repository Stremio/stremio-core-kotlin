use std::convert::TryFrom;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, PartialEq, Eq, Hash, EnumIter)]
#[allow(non_camel_case_types)]
pub enum KotlinClassName {
    String,
    Integer,
    Double,
    Date,
    ArrayList,
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
    ExtraValue,
    MetaItem,
    Video,
    Video_SeriesInfo,
    MetaItemPreview,
    MetaItemBehaviorHints,
    Stream,
    StreamBehaviorHints,
    StreamSource,
    StreamSource_Url,
    StreamSource_YouTube,
    StreamSource_Torrent,
    StreamSource_External,
    StreamSource_PlayerFrame,
    Link,
    PosterShape,
    Loadable,
    Loadable_Loading,
    Loadable_Ready,
    Loadable_Error,
    ResourceLoadable,
    ResourceRequest,
    ResourcePath,
    CatalogsWithExtra,
    CatalogsWithExtra_Selected,
    StreamingServer,
    StreamingServer_Selected,
    StreamingServer_Settings,
    Ctx,
    Auth,
    LinkAuthKey,
    LinkCodeResponse,
    Profile,
    Settings,
    User,
    GDPRConsent,
    AuthRequest,
    AuthRequest_Login,
    AuthRequest_LoginWithToken,
    AuthRequest_Register,
    AuthRequest_GDPRConsentRequest,
    LinkModel,
    ContinueWatchingPreview,
    LibraryItem,
    LibraryItemState,
    LibraryItemBehaviorHints,
    MetaDetails,
    MetaDetails_Selected,
    CatalogWithFilters,
    CatalogWithFilters_Selected,
    CatalogWithFilters_Selectable,
    CatalogWithFilters_SelectableType,
    CatalogWithFilters_SelectableCatalog,
    CatalogWithFilters_SelectableExtra,
    CatalogWithFilters_SelectableExtraOption,
    CatalogWithFilters_SelectablePage,
    LibraryByType,
    LibraryWithFilters,
    LibraryWithFilters_Selected,
    LibraryWithFilters_LibraryRequest,
    LibraryWithFilters_Sort,
    LibraryWithFilters_Selectable,
    LibraryWithFilters_SelectableType,
    LibraryWithFilters_SelectableSort,
    LibraryWithFilters_SelectablePage,
    ExternalPlayerLink,
    LibraryItemDeepLinks,
    MetaItemDeepLinks,
    LibraryDeepLinks,
    DiscoverDeepLinks,
    StreamDeepLinks,
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
            KotlinClassName::Integer => "java/lang/Integer",
            KotlinClassName::Double => "java/lang/Double",
            KotlinClassName::Date => "java/util/Date",
            KotlinClassName::ArrayList => "java/util/ArrayList",
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
            KotlinClassName::ExtraValue => "com/stremio/core/types/addon/ExtraValue",
            KotlinClassName::MetaItem => "com/stremio/core/types/resource/MetaItem",
            KotlinClassName::Video => "com/stremio/core/types/resource/Video",
            KotlinClassName::Video_SeriesInfo => "com/stremio/core/types/resource/Video$SeriesInfo",
            KotlinClassName::MetaItemPreview => "com/stremio/core/types/resource/MetaItemPreview",
            KotlinClassName::MetaItemBehaviorHints => {
                "com/stremio/core/types/resource/MetaItemBehaviorHints"
            }
            KotlinClassName::Stream => "com/stremio/core/types/resource/Stream",
            KotlinClassName::StreamBehaviorHints => {
                "com/stremio/core/types/resource/StreamBehaviorHints"
            }
            KotlinClassName::StreamSource => "com/stremio/core/types/resource/StreamSource",
            KotlinClassName::StreamSource_Url => "com/stremio/core/types/resource/StreamSource$Url",
            KotlinClassName::StreamSource_YouTube => {
                "com/stremio/core/types/resource/StreamSource$YouTube"
            }
            KotlinClassName::StreamSource_Torrent => {
                "com/stremio/core/types/resource/StreamSource$Torrent"
            }
            KotlinClassName::StreamSource_External => {
                "com/stremio/core/types/resource/StreamSource$External"
            }
            KotlinClassName::StreamSource_PlayerFrame => {
                "com/stremio/core/types/resource/StreamSource$PlayerFrame"
            }
            KotlinClassName::Link => "com/stremio/core/types/resource/Link",
            KotlinClassName::PosterShape => "com/stremio/core/types/resource/PosterShape",
            KotlinClassName::Loadable => "com/stremio/core/models/common/Loadable",
            KotlinClassName::Loadable_Loading => "com/stremio/core/models/common/Loadable$Loading",
            KotlinClassName::Loadable_Ready => "com/stremio/core/models/common/Loadable$Ready",
            KotlinClassName::Loadable_Error => "com/stremio/core/models/common/Loadable$Error",
            KotlinClassName::ResourceLoadable => "com/stremio/core/models/common/ResourceLoadable",
            KotlinClassName::ResourceRequest => "com/stremio/core/types/addon/ResourceRequest",
            KotlinClassName::ResourcePath => "com/stremio/core/types/addon/ResourcePath",
            KotlinClassName::CatalogsWithExtra => "com/stremio/core/models/CatalogsWithExtra",
            KotlinClassName::CatalogsWithExtra_Selected => {
                "com/stremio/core/models/CatalogsWithExtra$Selected"
            }
            KotlinClassName::StreamingServer => "com/stremio/core/models/StreamingServer",
            KotlinClassName::StreamingServer_Selected => {
                "com/stremio/core/models/StreamingServer$Selected"
            }
            KotlinClassName::StreamingServer_Settings => {
                "com/stremio/core/models/StreamingServer$Settings"
            }
            KotlinClassName::Ctx => "com/stremio/core/models/Ctx",
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
            KotlinClassName::LinkAuthKey => "com/stremio/core/types/api/LinkAuthKey",
            KotlinClassName::LinkCodeResponse => "com/stremio/core/types/api/LinkCodeResponse",
            KotlinClassName::Auth => "com/stremio/core/types/profile/Auth",
            KotlinClassName::Profile => "com/stremio/core/types/profile/Profile",
            KotlinClassName::Settings => "com/stremio/core/types/profile/Settings",
            KotlinClassName::User => "com/stremio/core/types/profile/User",
            KotlinClassName::GDPRConsent => "com/stremio/core/types/profile/GDPRConsent",
            KotlinClassName::LinkModel => "com/stremio/core/models/Link",
            KotlinClassName::ContinueWatchingPreview => {
                "com/stremio/core/models/ContinueWatchingPreview"
            }
            KotlinClassName::LibraryItem => "com/stremio/core/types/library/LibraryItem",
            KotlinClassName::LibraryItemState => "com/stremio/core/types/library/LibraryItemState",
            KotlinClassName::LibraryItemBehaviorHints => {
                "com/stremio/core/types/library/LibraryItemBehaviorHints"
            }
            KotlinClassName::MetaDetails => "com/stremio/core/models/MetaDetails",
            KotlinClassName::MetaDetails_Selected => "com/stremio/core/models/MetaDetails$Selected",
            KotlinClassName::CatalogWithFilters => "com/stremio/core/models/CatalogWithFilters",
            KotlinClassName::CatalogWithFilters_Selected => {
                "com/stremio/core/models/CatalogWithFilters$Selected"
            }
            KotlinClassName::CatalogWithFilters_Selectable => {
                "com/stremio/core/models/CatalogWithFilters$Selectable"
            }
            KotlinClassName::CatalogWithFilters_SelectableType => {
                "com/stremio/core/models/CatalogWithFilters$SelectableType"
            }
            KotlinClassName::CatalogWithFilters_SelectableCatalog => {
                "com/stremio/core/models/CatalogWithFilters$SelectableCatalog"
            }
            KotlinClassName::CatalogWithFilters_SelectableExtra => {
                "com/stremio/core/models/CatalogWithFilters$SelectableExtra"
            }
            KotlinClassName::CatalogWithFilters_SelectableExtraOption => {
                "com/stremio/core/models/CatalogWithFilters$SelectableExtraOption"
            }
            KotlinClassName::CatalogWithFilters_SelectablePage => {
                "com/stremio/core/models/CatalogWithFilters$SelectablePage"
            }
            KotlinClassName::LibraryByType => "com/stremio/core/models/LibraryByType",
            KotlinClassName::LibraryWithFilters => "com/stremio/core/models/LibraryWithFilters",
            KotlinClassName::LibraryWithFilters_Selected => {
                "com/stremio/core/models/LibraryWithFilters$Selected"
            }
            KotlinClassName::LibraryWithFilters_LibraryRequest => {
                "com/stremio/core/models/LibraryWithFilters$LibraryRequest"
            }
            KotlinClassName::LibraryWithFilters_Sort => {
                "com/stremio/core/models/LibraryWithFilters$Sort"
            }
            KotlinClassName::LibraryWithFilters_Selectable => {
                "com/stremio/core/models/LibraryWithFilters$Selectable"
            }
            KotlinClassName::LibraryWithFilters_SelectableType => {
                "com/stremio/core/models/LibraryWithFilters$SelectableType"
            }
            KotlinClassName::LibraryWithFilters_SelectableSort => {
                "com/stremio/core/models/LibraryWithFilters$SelectableSort"
            }
            KotlinClassName::LibraryWithFilters_SelectablePage => {
                "com/stremio/core/models/LibraryWithFilters$SelectablePage"
            }
            KotlinClassName::ExternalPlayerLink => "com/stremio/core/deeplinks/ExternalPlayerLink",
            KotlinClassName::LibraryItemDeepLinks => {
                "com/stremio/core/deeplinks/LibraryItemDeepLinks"
            }
            KotlinClassName::MetaItemDeepLinks => "com/stremio/core/deeplinks/MetaItemDeepLinks",
            KotlinClassName::LibraryDeepLinks => "com/stremio/core/deeplinks/LibraryDeepLinks",
            KotlinClassName::DiscoverDeepLinks => "com/stremio/core/deeplinks/DiscoverDeepLinks",
            KotlinClassName::StreamDeepLinks => "com/stremio/core/deeplinks/StreamDeepLinks",
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
