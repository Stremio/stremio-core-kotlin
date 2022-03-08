package com.stremio.core.models

import com.stremio.core.models.common.Loadable
import com.stremio.core.types.api.LinkCodeResponse

data class Link<T>(
    val code: Loadable<LinkCodeResponse>?,
    val data: Loadable<T>?
)
