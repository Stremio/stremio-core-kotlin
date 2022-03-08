package com.stremio.core.types.profile

data class GDPRConsent(
    val tos: Boolean,
    val privacy: Boolean,
    val marketing: Boolean,
)
