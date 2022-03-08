package com.stremio.core.types.api

import com.stremio.core.types.profile.GDPRConsent
import java.util.*

sealed class AuthRequest {
    data class Login(val email: String, val password: String, val facebook: Boolean) : AuthRequest()
    data class LoginWithToken(val token: String) : AuthRequest()
    data class Register(
        val email: String,
        val password: String,
        val gdprConsentRequest: GDPRConsentRequest
    ) : AuthRequest()

    data class GDPRConsentRequest(val gdprConsent: GDPRConsent, val time: Date, val from: String)
}
