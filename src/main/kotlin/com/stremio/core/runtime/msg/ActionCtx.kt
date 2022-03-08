package com.stremio.core.runtime.msg

import com.stremio.core.types.api.AuthRequest
import com.stremio.core.types.profile.Settings
import com.stremio.core.types.resource.MetaItemPreview

sealed class ActionCtx {
    data class Authenticate(val args: AuthRequest) : ActionCtx()
    class Logout : ActionCtx()
    data class UpdateSettings(val args: Settings) : ActionCtx()
    data class AddToLibrary(val args: MetaItemPreview) : ActionCtx()
    data class RemoveFromLibrary(val args: String) : ActionCtx()
    data class RewindLibraryItem(val args: String) : ActionCtx()
    class PushUserToAPI : ActionCtx()
    class PullUserFromAPI : ActionCtx()
    class PushAddonsToAPI : ActionCtx()
    class PullAddonsFromAPI : ActionCtx()
    class SyncLibraryWithAPI : ActionCtx()

}
