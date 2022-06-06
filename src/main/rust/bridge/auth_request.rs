use std::convert::TryInto;

use chrono::{DateTime, Utc};
use jni::objects::JObject;
use jni::JNIEnv;
use stremio_core::types::api::{AuthRequest, GDPRConsentRequest};
use stremio_core::types::profile::GDPRConsent;

use crate::bridge::{TryFromKotlin, TryIntoKotlin};
use crate::env::{AndroidEnv, KotlinClassName};
use crate::jni_ext::JObjectExt;

impl<'a> TryIntoKotlin<'a, ()> for GDPRConsentRequest {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        let gdpr_consent = self.gdpr_consent.try_into_kotlin(&(), env)?.auto_local(env);
        let time = self.time.try_into_kotlin(&(), env)?.auto_local(env);
        let from = self.from.try_into_kotlin(&(), env)?.auto_local(env);
        env.new_object(
            classes
                .get(&KotlinClassName::AuthRequest_GDPRConsentRequest)
                .unwrap(),
            format!(
                "(L{};L{};L{};)V",
                KotlinClassName::GDPRConsent.value(),
                KotlinClassName::Date.value(),
                KotlinClassName::String.value()
            ),
            &[
                gdpr_consent.as_obj().into(),
                time.as_obj().into(),
                from.as_obj().into(),
            ],
        )
    }
}

impl<'a> TryIntoKotlin<'a, ()> for AuthRequest {
    #[inline]
    fn try_into_kotlin(&self, _args: &(), env: &JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        let classes = AndroidEnv::kotlin_classes().unwrap();
        match self {
            AuthRequest::Login {
                email,
                password,
                facebook,
            } => {
                let email = email.try_into_kotlin(&(), env)?.auto_local(env);
                let password = password.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::AuthRequest_Login).unwrap(),
                    format!(
                        "(L{};L{};Z)V",
                        KotlinClassName::String.value(),
                        KotlinClassName::String.value()
                    ),
                    &[
                        email.as_obj().into(),
                        password.as_obj().into(),
                        (*facebook).into(),
                    ],
                )
            }
            AuthRequest::LoginWithToken { token } => {
                let token = token.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes
                        .get(&KotlinClassName::AuthRequest_LoginWithToken)
                        .unwrap(),
                    format!("(L{};)V", KotlinClassName::String.value()),
                    &[token.as_obj().into()],
                )
            }
            AuthRequest::Register {
                email,
                password,
                gdpr_consent,
            } => {
                let email = email.try_into_kotlin(&(), env)?.auto_local(env);
                let password = password.try_into_kotlin(&(), env)?.auto_local(env);
                let gdpr_consent = gdpr_consent.try_into_kotlin(&(), env)?.auto_local(env);
                env.new_object(
                    classes.get(&KotlinClassName::AuthRequest_Register).unwrap(),
                    format!(
                        "(L{};L{};L{};)V",
                        KotlinClassName::String.value(),
                        KotlinClassName::String.value(),
                        KotlinClassName::AuthRequest_GDPRConsentRequest.value()
                    ),
                    &[
                        email.as_obj().into(),
                        password.as_obj().into(),
                        gdpr_consent.as_obj().into(),
                    ],
                )
            }
        }
    }
}

impl TryFromKotlin for GDPRConsentRequest {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let gdpr_consent = env
            .call_method(
                value,
                "getGdprConsent",
                format!("()L{};", KotlinClassName::GDPRConsent.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let gdpr_consent = GDPRConsent::try_from_kotlin(gdpr_consent.as_obj(), env)?;
        let time = env
            .call_method(
                value,
                "getTime",
                format!("()L{};", KotlinClassName::Date.value()),
                &[],
            )?
            .l()?
            .auto_local(env);
        let time = DateTime::<Utc>::try_from_kotlin(time.as_obj(), env)?;
        let from = env
            .call_method(value, "getFrom", "()Ljava/lang/String;", &[])?
            .l()?
            .auto_local(env);
        let from = String::try_from_kotlin(from.as_obj(), env)?;
        Ok(GDPRConsentRequest {
            gdpr_consent,
            time,
            from,
        })
    }
}

impl TryFromKotlin for AuthRequest {
    fn try_from_kotlin<'a>(value: JObject<'a>, env: &JNIEnv<'a>) -> jni::errors::Result<Self> {
        let class_name = value.get_class_name(env)?;
        match class_name.replace(".", "/").try_into() {
            Ok(KotlinClassName::AuthRequest_Login) => {
                let email = env
                    .call_method(value, "getEmail", "()Ljava/lang/String;", &[])?
                    .l()?
                    .auto_local(env);
                let email = String::try_from_kotlin(email.as_obj(), env)?;
                let password = env
                    .call_method(value, "getPassword", "()Ljava/lang/String;", &[])?
                    .l()?
                    .auto_local(env);
                let password = String::try_from_kotlin(password.as_obj(), env)?;
                let facebook = env.call_method(value, "getFacebook", "()Z", &[])?.z()?;
                Ok(AuthRequest::Login {
                    email,
                    password,
                    facebook,
                })
            }
            Ok(KotlinClassName::AuthRequest_LoginWithToken) => {
                let token = env
                    .call_method(value, "getToken", "()Ljava/lang/String;", &[])?
                    .l()?
                    .auto_local(env);
                let token = String::try_from_kotlin(token.as_obj(), env)?;
                Ok(AuthRequest::LoginWithToken { token })
            }
            Ok(KotlinClassName::AuthRequest_Register) => {
                let email = env
                    .call_method(value, "getEmail", "()Ljava/lang/String;", &[])?
                    .l()?
                    .auto_local(env);
                let email = String::try_from_kotlin(email.as_obj(), env)?;
                let password = env
                    .call_method(value, "getPassword", "()Ljava/lang/String;", &[])?
                    .l()?
                    .auto_local(env);
                let password = String::try_from_kotlin(password.as_obj(), env)?;
                let gdpr_consent = env
                    .call_method(
                        value,
                        "getGdprConsentRequest",
                        format!(
                            "()L{};",
                            KotlinClassName::AuthRequest_GDPRConsentRequest.value()
                        ),
                        &[],
                    )?
                    .l()?
                    .auto_local(env);
                let gdpr_consent = GDPRConsentRequest::try_from_kotlin(gdpr_consent.as_obj(), env)?;
                Ok(AuthRequest::Register {
                    email,
                    password,
                    gdpr_consent,
                })
            }
            _ => panic!("AuthRequest not supported: {}", class_name),
        }
    }
}
