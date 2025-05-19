use stremio_core::types::api::AuthRequest;

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<AuthRequest> for types::AuthRequest {
    fn from_protobuf(&self) -> AuthRequest {
        match &self.r#type {
            Some(types::auth_request::Type::Login(login)) => AuthRequest::Login {
                email: login.email.to_owned(),
                password: login.password.to_owned(),
                facebook: login.facebook.to_owned(),
            },
            Some(types::auth_request::Type::LoginWithToken(login_with_token)) => {
                AuthRequest::LoginWithToken {
                    token: login_with_token.token.to_owned(),
                }
            }
            Some(types::auth_request::Type::Facebook(facebook)) => AuthRequest::Facebook {
                token: facebook.token.to_owned(),
            },
            Some(types::auth_request::Type::Apple(apple)) => AuthRequest::Apple {
                token: apple.token.to_owned(),
                sub: apple.sub.to_owned(),
                email: apple.email.to_owned(),
                name: apple.name.to_owned(),
            },
            Some(types::auth_request::Type::Register(register)) => AuthRequest::Register {
                email: register.email.to_owned(),
                password: register.password.to_owned(),
                gdpr_consent: register.gdpr_consent.from_protobuf(),
            },
            None => unimplemented!("AuthRequest must be present"),
        }
    }
}

impl ToProtobuf<types::AuthRequest, ()> for AuthRequest {
    fn to_protobuf<E: stremio_core::runtime::Env + 'static>(
        &self,
        _args: &(),
    ) -> types::AuthRequest {
        let request = match self {
            AuthRequest::Login {
                email,
                password,
                facebook,
            } => types::auth_request::Type::Login(types::auth_request::Login {
                email: email.to_owned(),
                password: password.to_owned(),
                facebook: facebook.to_owned(),
            }),
            AuthRequest::LoginWithToken { token } => {
                types::auth_request::Type::LoginWithToken(types::auth_request::LoginWithToken {
                    token: token.to_owned(),
                })
            }
            AuthRequest::Facebook { token } => {
                types::auth_request::Type::Facebook(types::auth_request::Facebook {
                    token: token.to_owned(),
                })
            }
            AuthRequest::Apple {
                token,
                sub,
                email,
                name,
            } => types::auth_request::Type::Apple(types::auth_request::Apple {
                token: token.to_owned(),
                sub: sub.to_owned(),
                email: email.to_owned(),
                name: name.to_owned(),
            }),
            AuthRequest::Register {
                email,
                password,
                gdpr_consent,
            } => types::auth_request::Type::Register(types::auth_request::Register {
                email: email.to_owned(),
                password: password.to_owned(),
                gdpr_consent: gdpr_consent.to_protobuf::<E>(&()),
            }),
        };
        types::AuthRequest {
            r#type: Some(request),
        }
    }
}
