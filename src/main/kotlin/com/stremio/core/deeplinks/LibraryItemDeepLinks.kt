package com.stremio.core.deeplinks

data class LibraryItemDeepLinks(
    val metaDetailsVideos: String?,
    val metaDetailsStreams: String?,
    val player: String?,
    val externalPlayer: ExternalPlayerLink?
)
