package com.stremio.core.models

import com.stremio.core.models.common.ResourceLoadable
import com.stremio.core.types.addon.ResourcePath
import com.stremio.core.types.resource.MetaItem
import com.stremio.core.types.resource.Stream

data class MetaDetails(
    val selected: Selected?,
    val title: String?,
    val metaItem: ResourceLoadable<MetaItem>?,
    val streams: List<ResourceLoadable<List<Stream>>>
) {
    data class Selected(
        val metaPath: ResourcePath,
        val streamPath: ResourcePath?,
    )
}
