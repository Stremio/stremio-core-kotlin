package com.stremio.core.types.addon

data class ResourcePath(
    val resource: String,
    val type: String,
    val id: String,
    val extra: List<ExtraValue>
) {
    companion object {
        const val META = "meta"
        const val STREAM = "stream"
        const val SUBTITLE = "subtitle"
    }
}
