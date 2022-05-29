use crate::bridge::TryIntoKotlin;
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::models::ctx::CtxError;
use stremio_core::runtime::msg::Event;
use stremio_core::runtime::RuntimeEvent;

impl<'a> TryIntoKotlin<'a, ()> for Event {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        match self {
            Event::ProfilePushedToStorage { uid } => {
                let uid = uid.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_ProfilePushedToStorage)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[uid.as_obj().into()],
                )
            }
            Event::LibraryItemsPushedToStorage { ids } => {
                let ids = ids.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibraryItemsPushedToStorage)
                        .unwrap(),
                    "(Ljava/util/List;)V",
                    &[ids.as_obj().into()],
                )
            }
            Event::UserPulledFromAPI { uid } => {
                let uid = uid.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_UserPulledFromAPI)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[uid.as_obj().into()],
                )
            }
            Event::UserPushedToAPI { uid } => {
                let uid = uid.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_UserPushedToAPI)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[uid.as_obj().into()],
                )
            }
            Event::AddonsPulledFromAPI { transport_urls } => {
                let transport_urls = transport_urls.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_AddonsPulledFromAPI)
                        .unwrap(),
                    "(Ljava/util/List;)V",
                    &[transport_urls.as_obj().into()],
                )
            }
            Event::AddonsPushedToAPI { transport_urls } => {
                let transport_urls = transport_urls.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_AddonsPushedToAPI)
                        .unwrap(),
                    "(Ljava/util/List;)V",
                    &[transport_urls.as_obj().into()],
                )
            }
            Event::UserAuthenticated { auth_request } => {
                let auth_request = auth_request.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_UserAuthenticated)
                        .unwrap(),
                    format!("(L{};)V", KotlinClassName::AuthRequest.value()),
                    &[auth_request.as_obj().into()],
                )
            }
            Event::UserLoggedOut { uid } => {
                let uid = uid.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Event_UserLoggedOut).unwrap(),
                    "(Ljava/lang/String;)V",
                    &[uid.as_obj().into()],
                )
            }
            Event::SessionDeleted { auth_key } => {
                let auth_key = auth_key.0.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Event_SessionDeleted).unwrap(),
                    "(Ljava/lang/String;)V",
                    &[auth_key.as_obj().into()],
                )
            }
            Event::AddonInstalled { transport_url, id } => {
                let transport_url = transport_url.try_into_kotlin(&(), env)?.auto_local(env);
                let id = id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Event_AddonInstalled).unwrap(),
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    &[transport_url.as_obj().into(), id.as_obj().into()],
                )
            }
            Event::AddonUpgraded { transport_url, id } => {
                let transport_url = transport_url.try_into_kotlin(&(), env)?.auto_local(env);
                let id = id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Event_AddonUpgraded).unwrap(),
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    &[transport_url.as_obj().into(), id.as_obj().into()],
                )
            }
            Event::AddonUninstalled { transport_url, id } => {
                let transport_url = transport_url.try_into_kotlin(&(), env)?.auto_local(env);
                let id = id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_AddonUninstalled)
                        .unwrap(),
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    &[transport_url.as_obj().into(), id.as_obj().into()],
                )
            }
            Event::LibraryItemAdded { id } => {
                let id = id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibraryItemAdded)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[id.as_obj().into()],
                )
            }
            Event::LibraryItemRemoved { id } => {
                let id = id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibraryItemRemoved)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[id.as_obj().into()],
                )
            }
            Event::LibraryItemRewinded { id } => {
                let id = id.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibraryItemRewinded)
                        .unwrap(),
                    "(Ljava/lang/String;)V",
                    &[id.as_obj().into()],
                )
            }
            Event::LibrarySyncWithAPIPlanned { plan } => {
                let plan = plan.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibrarySyncWithAPIPlanned)
                        .unwrap(),
                    format!("(L{};)V", KotlinClassName::Pair.value()),
                    &[plan.as_obj().into()],
                )
            }
            Event::LibraryItemsPushedToAPI { ids } => {
                let ids = ids.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibraryItemsPushedToAPI)
                        .unwrap(),
                    "(Ljava/util/List;)V",
                    &[ids.as_obj().into()],
                )
            }
            Event::LibraryItemsPulledFromAPI { ids } => {
                let ids = ids.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_LibraryItemsPulledFromAPI)
                        .unwrap(),
                    "(Ljava/util/List;)V",
                    &[ids.as_obj().into()],
                )
            }
            Event::SettingsUpdated { settings } => {
                let settings = settings.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::Event_SettingsUpdated)
                        .unwrap(),
                    format!("(L{};)V", KotlinClassName::Profile_Settings.value()),
                    &[settings.as_obj().into()],
                )
            }
            Event::Error { error, source } => {
                let error = match error {
                    CtxError::API(error) => error.message.to_owned(),
                    CtxError::Env(error) => error.message(),
                    CtxError::Other(error) => error.message(),
                };
                let error = error.try_into_kotlin(&(), env)?.auto_local(env);
                let source = source.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::Event_Error).unwrap(),
                    format!("(Ljava/lang/String;L{};)V", KotlinClassName::Event.value()),
                    &[error.as_obj().into(), source.as_obj().into()],
                )
            }
        }
    }
}

impl<'a> TryIntoKotlin<'a, ()> for RuntimeEvent {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        match self {
            RuntimeEvent::NewState => env.new_object(
                classes
                    .get(&KotlinClassName::RuntimeEvent_NewState)
                    .unwrap(),
                "()V",
                &[],
            ),
            RuntimeEvent::CoreEvent(event) => {
                let event = event.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::RuntimeEvent_CoreEvent)
                        .unwrap(),
                    format!("(L{};)V", KotlinClassName::Event.value()),
                    &[event.as_obj().into()],
                )
            }
        }
    }
}
