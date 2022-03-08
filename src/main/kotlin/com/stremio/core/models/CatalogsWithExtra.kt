package com.stremio.core.models

import com.stremio.core.models.common.ResourceLoadable
import com.stremio.core.types.addon.ExtraValue
import com.stremio.core.types.resource.MetaItemPreview

data class CatalogsWithExtra(
    val selected: Selected?,
    val catalogs: List<ResourceLoadable<List<MetaItemPreview>>>
) {
    data class Selected(val extra: List<ExtraValue>)
}
