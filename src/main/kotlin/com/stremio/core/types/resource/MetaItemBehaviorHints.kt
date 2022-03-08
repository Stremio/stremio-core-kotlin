package com.stremio.core.types.resource

data class MetaItemBehaviorHints(
    val defaultVideoId: String?,
    val featuredVideoId: String?,
    val hasScheduledVideos: Boolean
)
