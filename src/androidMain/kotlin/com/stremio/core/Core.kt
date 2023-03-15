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

    private external fun initializeNative(storage: Storage): ByteArray?

    private external fun dispatchNative(actionProtobuf: ByteArray)

    private external fun decodeStreamDataNative(streamData: String): ByteArray?

    external fun getStateNative(field: Field): ByteArray

    external fun sendNextAnalyticsBatch()

    fun initialize(storage: Storage): EnvError? {
        return initializeNative(storage)
            ?.let { EnvError.decodeFromByteArray(it) }
    }

    fun dispatch(action: Action, field: Field?) {
        val actionProtobuf = RuntimeAction(field, action).encodeToByteArray()
        dispatchNative(actionProtobuf)
    }

    @Suppress("UNCHECKED_CAST")
    inline fun <reified T : Message> getState(field: Field): T {
        val protobuf = getStateNative(field)
        val companion = T::class.companionObjectInstance as Message.Companion<T>
        return companion.decodeFromByteArray(protobuf)
    }

    fun decodeStreamData(streamData: String): Stream? {
        return decodeStreamDataNative(streamData)
            ?.let { Stream.decodeFromByteArray(it) }
    }

    @JvmStatic
    private fun onRuntimeEvent(eventProtobuf: ByteArray) {
        listeners.forEach {
            try {
                val event = RuntimeEvent.decodeFromByteArray(eventProtobuf)
                it.onEvent(event)
            } catch (e: Exception) {
                Log.e("Stremio", "Failed passing event: ", e)
            }
        }
    }
}
