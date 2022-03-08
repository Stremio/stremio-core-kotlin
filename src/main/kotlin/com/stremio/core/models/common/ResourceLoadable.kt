package com.stremio.core.models.common

import com.stremio.core.types.addon.ResourceRequest

data class ResourceLoadable<T>(
    val title: String?,
    val request: ResourceRequest,
    val content: Loadable<T>
)
