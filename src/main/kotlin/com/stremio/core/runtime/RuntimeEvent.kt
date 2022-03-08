package com.stremio.core.runtime

import com.stremio.core.runtime.msg.Event

sealed class RuntimeEvent {
    class NewState : RuntimeEvent()
    data class CoreEvent(val event: Event) : RuntimeEvent()
}
