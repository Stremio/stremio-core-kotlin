package com.stremio.core.models

import com.stremio.core.deeplinks.LibraryDeepLinks
import com.stremio.core.types.library.LibraryItem

data class LibraryWithFilters(
    val selected: Selected?,
    val selectable: Selectable,
    val catalog: List<LibraryItem>
) {
    data class Selected(
        val request: LibraryRequest
    )

    data class LibraryRequest(
        val type: String?,
        val sort: Sort,
        val page: UInt
    )

    enum class Sort(val value: String) {
        LastWatched("lastwatched"),
        Name("name"),
        TimesWatched("timeswatched"),
    }

    data class Selectable(
        val types: List<SelectableType>,
        val sorts: List<SelectableSort>,
        val prevPage: SelectablePage?,
        val nextPage: SelectablePage?
    )

    data class SelectableType(
        val type: String?,
        val selected: Boolean,
        val deepLinks: LibraryDeepLinks
    )

    data class SelectableSort(
        val sort: Sort,
        val selected: Boolean,
        val deepLinks: LibraryDeepLinks,
    )

    data class SelectablePage(
        val request: LibraryRequest,
        val deepLinks: LibraryDeepLinks,
    )
}
