package com.stremio.core.types.resource

data class StreamBehaviorHints(
    val notWebReady: Boolean,
    val bingeGroup: String?,
//    val countryWhitelist: List<String>?,
//    val headers: HashMap<String, String>
)
