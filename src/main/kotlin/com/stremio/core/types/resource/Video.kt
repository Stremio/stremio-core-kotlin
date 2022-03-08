package com.stremio.core.types.resource

import java.util.*

data class Video(
    val id: String,
    val title: String,
    val released: Date?,
    val overview: String?,
    val thumbnail: String?,
    val streams: List<Stream>,
    val seriesInfo: SeriesInfo?,
//    val trailerStreams: List<Stream>,
) {
    data class SeriesInfo(
        val season: UInt,
        val episode: UInt
    )
}
