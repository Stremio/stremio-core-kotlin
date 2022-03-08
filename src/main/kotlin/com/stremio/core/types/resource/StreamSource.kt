package com.stremio.core.types.resource

sealed class StreamSource {
    data class Url(val url: String) : StreamSource()
    data class YouTube(val ytId: String) : StreamSource()
    data class Torrent(val infoHash: String, val fileIdx: UInt?, val announce: List<String>) :
        StreamSource()

    data class External(val externalUrl: String) : StreamSource()
    data class PlayerFrame(val playerFrameUrl: String) : StreamSource()
}
