package com.stremio.core.models

import com.stremio.core.models.common.Loadable

data class StreamingServer(
    val selected: Selected,
    val settings: Loadable<Settings>,
    val baseUrl: Loadable<String>
) {
    data class Settings(
        val appPath: String,
        val cacheRoot: String,
        val serverVersion: String,
        val cacheSize: Double?,
        val btMaxConnections: ULong,
        val btHandshakeTimeout: ULong,
        val btRequestTimeout: ULong,
        val btDownloadSpeedSoftLimit: Double,
        val btDownloadSpeedHardLimit: Double,
        val btMinPeersForStable: ULong,
    )

    data class Selected(val transportUrl: String)
}
