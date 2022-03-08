package com.stremio.core.types.resource

import com.stremio.core.deeplinks.StreamDeepLinks

data class Stream(
    val source: StreamSource,
    val name: String?,
    val description: String?,
    val thumbnail: String?,
//    val subtitles: List<Subtitles>
    val behaviorHints: StreamBehaviorHints,
    val deepLinks: StreamDeepLinks
)
