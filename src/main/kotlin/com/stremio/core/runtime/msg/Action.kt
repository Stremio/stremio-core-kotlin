package com.stremio.core.runtime.msg

sealed class Action {
    data class Ctx(val args: ActionCtx) : Action()
    data class Link(val args: ActionLink) : Action()
    data class StreamingServer(val args: ActionStreamingServer) : Action()

    //    class Player(val args: ActionPlayer) : Action()
    data class Load(val args: ActionLoad) : Action()
    class Unload : Action()
}
