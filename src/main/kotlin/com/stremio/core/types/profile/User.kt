package com.stremio.core.types.profile

data class User(
    val id: String,
    val email: String,
    val fbId: String?,
    val avatar: String?,
    val gdprConsent: GDPRConsent
)
