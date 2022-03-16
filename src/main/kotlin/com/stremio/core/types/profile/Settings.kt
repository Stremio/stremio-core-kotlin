package com.stremio.core.types.profile

data class Settings(
    val interfaceLanguage: String,
    val streamingServerUrl: String,
    val bingeWatching: Boolean,
    val playInBackground: Boolean,
    val playInExternalPlayer: Boolean,
    val hardwareDecoding: Boolean,
    val subtitlesLanguage: String,
    val subtitlesSize: Int,
    val subtitlesFont: String,
    val subtitlesBold: Boolean,
    val subtitlesOffset: Int,
    val subtitlesTextColor: String,
    val subtitlesBackgroundColor: String,
    val subtitlesOutlineColor: String,
    val seekTimeDuration: Long
)
