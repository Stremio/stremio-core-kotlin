package com.stremio.core

import com.stremio.core.runtime.EnvError
import com.stremio.core.runtime.RuntimeEvent
import com.stremio.core.runtime.msg.Action
import com.stremio.core.types.resource.Stream
import pbandk.Message

expect object Core {
    fun interface EventListener {
        fun onEvent(event: RuntimeEvent)
    }
    fun addEventListener(listener: EventListener)
    fun removeEventListener(listener: EventListener)
    fun initialize(storage: Storage): EnvError?
    fun dispatch(action: Action, field: Field?)
    fun decodeStreamData(streamData: String): Stream?
    inline fun <reified T : Message> getState(field: Field): T
}
