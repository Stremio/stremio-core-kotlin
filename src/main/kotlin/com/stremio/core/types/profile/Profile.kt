package com.stremio.core.types.profile

typealias UID = String?

data class Profile(
    val auth: Auth?,
//    val addons: List<Descriptor>,
    val settings: Settings,
)
