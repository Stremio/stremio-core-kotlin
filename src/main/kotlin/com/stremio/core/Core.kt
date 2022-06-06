package com.stremio.core

import android.util.Log
import com.stremio.core.runtime.EnvError
import com.stremio.core.runtime.RuntimeEvent
import com.stremio.core.runtime.msg.Action
import com.stremio.core.types.resource.Stream
import pbandk.Message
import pbandk.decodeFromByteArray
import java.util.*
import java.util.concurrent.ConcurrentHashMap
import kotlin.reflect.full.companionObjectInstance

object Core {
    init {
        System.loadLibrary("stremio_core_android")
    }

    fun interface EventListener {
        fun onEvent(event: RuntimeEvent)
    }

    private val listeners = Collections.newSetFromMap(ConcurrentHashMap<EventListener, Boolean>())

    fun addEventListener(listener: EventListener) {
        listeners.add(listener)
    }

    fun removeEventListener(listener: EventListener) {
        listeners.remove(listener)
    }

    external fun initialize(storage: Storage): EnvError?

    external fun dispatch(action: Action, field: Field?)

    external fun getStateBinary(field: Field): ByteArray

    private external fun decodeStreamDataBinary(streamData: String): ByteArray

    @Suppress("UNCHECKED_CAST")
    inline fun <reified T : Message> getState(field: Field): T {
        val protobuf = getStateBinary(field)
        val companion = T::class.companionObjectInstance as Message.Companion<T>
        return companion.decodeFromByteArray(protobuf)
    }

    fun decodeStreamData(streamData: String): Stream {
        val decodedStreamProtobuf = decodeStreamDataBinary(streamData)
        return Stream.decodeFromByteArray(decodedStreamProtobuf)
    }

    @JvmStatic
    private fun onRuntimeEvent(event: RuntimeEvent) {
        listeners.forEach {
            try {
                it.onEvent(event)
            } catch (e: Exception) {
                Log.e("Stremio", "Failed passing event: ", e)
            }
        }
    }
}
