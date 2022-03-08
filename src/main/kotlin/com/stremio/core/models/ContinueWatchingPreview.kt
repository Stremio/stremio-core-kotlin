package com.stremio.core.models

import com.stremio.core.types.library.LibraryItem

data class ContinueWatchingPreview(
    val libraryItems: List<LibraryItem>,
)
