package com.stremio.core.types.library

import com.stremio.core.deeplinks.LibraryItemDeepLinks
import com.stremio.core.types.resource.PosterShape

data class LibraryItem(
    val id: String,
    val type: String,
    val name: String,
    val poster: String?,
    val posterShape: PosterShape,
    val state: LibraryItemState,
    val behaviorHints: LibraryItemBehaviorHints,
    val deepLinks: LibraryItemDeepLinks
)
