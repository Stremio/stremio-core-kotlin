package com.stremio.core

import android.util.Log
import com.stremio.core.runtime.EnvError
import com.stremio.core.runtime.RuntimeEvent
import com.stremio.core.runtime.msg.Action
import com.stremio.core.runtime.msg.RuntimeAction
import com.stremio.core.types.resource.Stream
import pbandk.Message
import pbandk.decodeFromByteArray
import pbandk.encodeToByteArray
import java.util.*
import java.util.concurrent.ConcurrentHashMap
import kotlin.reflect.full.companionObjectInstance

actual object Core {
    init {
        System.loadLibrary("stremio_core_kotlin")
    }

    actual fun interface EventListener {
        actual fun onEvent(event: RuntimeEvent)
    }

    private val listeners = Collections.newSetFromMap(ConcurrentHashMap<EventListener, Boolean>())

    actual fun addEventListener(listener: EventListener) {
        listeners.add(listener)
    }

    actual fun removeEventListener(listener: EventListener) {
        listeners.remove(listener)
    }

    private external fun initializeNative(storage: Storage): ByteArray?

    private external fun dispatchNative(actionProtobuf: ByteArray)

    private external fun decodeStreamDataNative(streamData: String): ByteArray?

    external fun getStateNative(field: Field): ByteArray

    external fun sendNextAnalyticsBatch()

    actual fun initialize(storage: Storage): EnvError? {
        return initializeNative(storage)
            ?.let { EnvError.decodeFromByteArray(it) }
    }

    actual fun dispatch(action: Action, field: Field?) {
        val actionProtobuf = RuntimeAction(field, action).encodeToByteArray()
        dispatchNative(actionProtobuf)
    }

    @Suppress("UNCHECKED_CAST")
    actual inline fun <reified T : Message> getState(field: Field): T {
        val protobuf = getStateNative(field)
        val companion = T::class.companionObjectInstance as Message.Companion<T>
        return companion.decodeFromByteArray(protobuf)
    }

    actual fun decodeStreamData(streamData: String): Stream? {
        return decodeStreamDataNative(streamData)
            ?.let { Stream.decodeFromByteArray(it) }
    }

    @JvmStatic
    private fun onRuntimeEvent(eventProtobuf: ByteArray) {
        for (listener in listeners) {
            try {
                val event = RuntimeEvent.decodeFromByteArray(eventProtobuf)
                listener.onEvent(event)
            } catch (e: Exception) {
                Log.e("Stremio", "Failed passing event: ", e)
            }
        }
    }
}
