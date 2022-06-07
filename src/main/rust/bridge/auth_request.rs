use stremio_core::types::api::{AuthRequest, GDPRConsentRequest};

use crate::bridge::{FromProtobuf, ToProtobuf};
use crate::protobuf::stremio::core::types;

impl FromProtobuf<GDPRConsentRequest> for types::GdprConsentRequest {
    fn from_protobuf(&self) -> GDPRConsentRequest {
        GDPRConsentRequest {
            gdpr_consent: self.gdpr_consent.from_protobuf(),
            time: self.time.from_protobuf(),
            from: self.from.to_owned(),
        }
    }
}

impl FromProtobuf<AuthRequest> for types::AuthRequest {
    fn from_protobuf(&self) -> AuthRequest {
        match &self.request {
            Some(types::auth_request::Request::Login(login)) => AuthRequest::Login {
                email: login.email.to_owned(),
                password: login.password.to_owned(),
                facebook: login.facebook.to_owned(),
            },
            Some(types::auth_request::Request::LoginWithToken(login_with_token)) => {
                AuthRequest::LoginWithToken {
                    token: login_with_token.token.to_owned(),
                }
            }
            Some(types::auth_request::Request::Register(register)) => AuthRequest::Register {
                email: register.email.to_owned(),
                password: register.password.to_owned(),
                gdpr_consent: register.gdpr_consent_request.from_protobuf(),
            },
            None => unimplemented!("AuthRequest must be present"),
        }
    }
}

impl ToProtobuf<types::GdprConsentRequest, ()> for GDPRConsentRequest {
    fn to_protobuf(&self, _args: &()) -> types::GdprConsentRequest {
        types::GdprConsentRequest {
            gdpr_consent: self.gdpr_consent.to_protobuf(&()),
            time: self.time.to_protobuf(&()),
            from: self.from.to_owned(),
        }
    }
}

impl ToProtobuf<types::AuthRequest, ()> for AuthRequest {
    fn to_protobuf(&self, _args: &()) -> types::AuthRequest {
        let request = match self {
            AuthRequest::Login {
                email,
                password,
                facebook,
            } => types::auth_request::Request::Login(types::auth_request::Login {
                email: email.to_owned(),
                password: password.to_owned(),
                facebook: facebook.to_owned(),
            }),
            AuthRequest::LoginWithToken { token } => {
                types::auth_request::Request::LoginWithToken(types::auth_request::LoginWithToken {
                    token: token.to_owned(),
                })
            }
            AuthRequest::Register {
                email,
                password,
                gdpr_consent,
            } => types::auth_request::Request::Register(types::auth_request::Register {
                email: email.to_owned(),
                password: password.to_owned(),
                gdpr_consent_request: gdpr_consent.to_protobuf(&()),
            }),
        };
        types::AuthRequest {
            request: Some(request),
        }
    }
}
