package com.stremio.core.runtime.msg

import com.stremio.core.models.StreamingServer

sealed class ActionStreamingServer {
    class Reload : ActionStreamingServer()
    data class UpdateSettings(val args: StreamingServer.Settings) : ActionStreamingServer()
}
