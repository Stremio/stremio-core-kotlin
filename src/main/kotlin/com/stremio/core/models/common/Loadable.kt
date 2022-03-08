package com.stremio.core.models.common

sealed class Loadable<T> {
    class Loading<T> : Loadable<T>()
    data class Ready<T>(val content: T) : Loadable<T>()
    data class Error<T>(val message: String) : Loadable<T>()
}
