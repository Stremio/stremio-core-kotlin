package com.stremio.core.types.resource

import com.stremio.core.deeplinks.MetaItemDeepLinks
import java.util.*

data class MetaItem(
    val id: String,
    val type: String,
    val name: String,
    val poster: String?,
    val background: String?,
    val logo: String?,
    val description: String?,
    val releaseInfo: String?,
    val runtime: String?,
    val released: Date?,
    val posterShape: PosterShape,
    val links: List<Link>,
    val trailerStreams: List<Stream>,
    val videos: List<Video>,
    val behaviorHints: MetaItemBehaviorHints,
    val deepLinks: MetaItemDeepLinks,
)
