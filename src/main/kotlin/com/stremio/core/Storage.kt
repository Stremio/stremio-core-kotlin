package com.stremio.core

interface Storage {
    sealed class Result<T> {
        class Ok<T>(val value: T) : Result<T>()
        class Err<T>(val message: String) : Result<T>()
    }

    fun get(key: String): Result<String?>
    fun set(key: String, value: String?): Result<Unit>
}
