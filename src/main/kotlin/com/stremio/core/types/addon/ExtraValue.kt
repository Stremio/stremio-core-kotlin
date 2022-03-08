package com.stremio.core.types.addon

data class ExtraValue(val name: String, val value: String) {
    companion object {
        const val SKIP = "skip"
        const val SEARCH = "search"
        const val GENRE = "genre"
    }
}
