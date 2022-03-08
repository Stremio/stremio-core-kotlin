package com.stremio.core.types.addon

data class ResourcePath(
    val resource: String,
    val type: String,
    val id: String,
    val extra: List<ExtraValue>
)
