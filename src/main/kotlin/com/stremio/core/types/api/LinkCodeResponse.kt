package com.stremio.core.types.api

data class LinkCodeResponse(
    val code: String,
    val link: String,
    val qrcode: String
)
