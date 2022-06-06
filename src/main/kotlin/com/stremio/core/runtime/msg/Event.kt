package com.stremio.core.runtime.msg

import com.stremio.core.types.api.AuthRequest
import com.stremio.core.types.profile.Profile

sealed class Event {
    data class ProfilePushedToStorage(val uid: String) : Event()
    data class LibraryItemsPushedToStorage(val ids: List<String>) : Event()
    data class UserPulledFromAPI(val uid: String) : Event()
    data class UserPushedToAPI(val uid: String) : Event()
    data class AddonsPulledFromAPI(val transportUrls: List<String>) : Event()
    data class AddonsPushedToAPI(val transportUrls: List<String>) : Event()
    data class LibrarySyncWithAPIPlanned(val plan: Pair<List<String>, List<String>>) : Event()
    data class LibraryItemsPushedToAPI(val ids: List<String>) : Event()
    data class LibraryItemsPulledFromAPI(val ids: List<String>) : Event()
    data class UserAuthenticated(val authRequest: AuthRequest) : Event()
    data class UserLoggedOut(val uid: String) : Event()
    data class SessionDeleted(val authKey: String) : Event()
    data class AddonInstalled(val transportUrl: String, val id: String) : Event()
    data class AddonUpgraded(val transportUrl: String, val id: String) : Event()
    data class AddonUninstalled(val transportUrl: String, val id: String) : Event()
    data class SettingsUpdated(val settings: Profile.Settings) : Event()
    data class LibraryItemAdded(val id: String) : Event()
    data class LibraryItemRemoved(val id: String) : Event()
    data class LibraryItemRewinded(val id: String) : Event()
    data class Error(val error: String, val source: Event) : Event()
}
