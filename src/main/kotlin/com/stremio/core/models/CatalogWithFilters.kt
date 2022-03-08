package com.stremio.core.models

import com.stremio.core.deeplinks.DiscoverDeepLinks
import com.stremio.core.models.common.ResourceLoadable
import com.stremio.core.types.addon.ResourceRequest
import com.stremio.core.types.resource.MetaItemPreview

data class CatalogWithFilters(
    val selected: Selected?,
    val selectable: Selectable,
    val catalog: ResourceLoadable<List<MetaItemPreview>>?
) {
    data class Selected(
        val request: ResourceRequest
    )

    data class Selectable(
        val types: List<SelectableType>,
        val catalogs: List<SelectableCatalog>,
        val extra: List<SelectableExtra>,
        val prevPage: SelectablePage?,
        val nextPage: SelectablePage?,
    )

    data class SelectableType(
        val type: String?,
        val selected: Boolean,
        val request: ResourceRequest,
        val deepLinks: DiscoverDeepLinks
    )

    data class SelectableCatalog(
        val name: String,
        val selected: Boolean,
        val request: ResourceRequest,
        val deepLinks: DiscoverDeepLinks
    )

    data class SelectableExtra(
        val name: String,
        val isRequired: Boolean,
        val options: List<SelectableExtraOption>
    )

    data class SelectableExtraOption(
        val value: String?,
        val selected: Boolean,
        val request: ResourceRequest,
        val deepLinks: DiscoverDeepLinks
    )

    data class SelectablePage(
        val request: ResourceRequest,
        val deepLinks: DiscoverDeepLinks
    )
}
