package com.stremio.core

import android.util.Log
import com.stremio.core.runtime.EnvError
import com.stremio.core.runtime.RuntimeEvent
import com.stremio.core.runtime.msg.Action
import java.util.*
import java.util.concurrent.ConcurrentHashMap

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

    external fun <T> getState(field: Field): T

    external fun dispatch(action: Action, field: Field?)

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
