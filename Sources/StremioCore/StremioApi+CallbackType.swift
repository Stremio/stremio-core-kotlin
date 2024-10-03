//
//  File.swift
//  
//
//  Created by Alvin on 17.06.24.
//

import Foundation
import SwiftProtobuf
import os.log

extension StremioApi {
    public class CallbackType {
        public static var error : Int = {
            var type = Stremio_Core_Runtime_Event()
            type.error.error = ""
            type.error.source = Stremio_Core_Runtime_Event()
            return type.getMessageTag
        }()
        
        public static var addonUninstalled : Int = {
            var type = Stremio_Core_Runtime_Event()
            type.addonUninstalled.transportURL = ""
            type.addonUninstalled.id = ""
            return type.getMessageTag
        }()
        
        public static var addonInstalled : Int = {
            var type = Stremio_Core_Runtime_Event()
            type.addonInstalled.transportURL = ""
            type.addonInstalled.id = ""
            return type.getMessageTag
        }()
        
        public static var addonUpgraded : Int = {
            var type = Stremio_Core_Runtime_Event()
            type.addonUpgraded.transportURL = ""
            type.addonUpgraded.id = ""
            return type.getMessageTag
        }()
        
        public static var settingsUpdated : Int = {
            var type = Stremio_Core_Runtime_Event()
            type.settingsUpdated.settings.interfaceLanguage = ""
            type.settingsUpdated.settings.streamingServerURL = ""
            type.settingsUpdated.settings.bingeWatching = false
            type.settingsUpdated.settings.playInBackground = false
            type.settingsUpdated.settings.hardwareDecoding = false
            type.settingsUpdated.settings.audioPassthrough = false
            type.settingsUpdated.settings.audioLanguage = ""
            type.settingsUpdated.settings.subtitlesLanguage = ""
            type.settingsUpdated.settings.subtitlesSize = 0
            type.settingsUpdated.settings.subtitlesFont = ""
            type.settingsUpdated.settings.subtitlesBold = false
            type.settingsUpdated.settings.subtitlesOffset = 0
            type.settingsUpdated.settings.subtitlesTextColor = ""
            type.settingsUpdated.settings.subtitlesBackgroundColor = ""
            type.settingsUpdated.settings.subtitlesOutlineColor = ""
            type.settingsUpdated.settings.subtitlesOpacity = 0
            type.settingsUpdated.settings.escExitFullscreen = false
            type.settingsUpdated.settings.seekTimeDuration = 0
            type.settingsUpdated.settings.seekShortTimeDuration = 0
            type.settingsUpdated.settings.pauseOnMinimize = false
            type.settingsUpdated.settings.secondaryAudioLanguage = ""
            type.settingsUpdated.settings.secondarySubtitlesLanguage = ""
            type.settingsUpdated.settings.playerType = ""
            type.settingsUpdated.settings.frameRateMatchingStrategy = .disabled
            type.settingsUpdated.settings.nextVideoNotificationDuration = 0
            type.settingsUpdated.settings.surroundSound = false
            return type.getMessageTag
        }()
        
        public static var userAuthenticated : Int = {
            var type = Stremio_Core_Runtime_Event()
            type.userAuthenticated.authRequest.loginWithToken.token = ""
            return type.getMessageTag
        }()
        
    }
    
    static internal func handleEvent<T>(callbackType: Int,
                        completionHandler: ((Result<T, Stremio_Core_Runtime_Event.Error>) -> Void)?) {
        if let completionHandler = completionHandler {
            Core.addEventListener(type: callbackType) { result in
                if case .error(_:) = result.type{
                    completionHandler(.failure(result.error))
                }
                else if let success = result.type?.get() as T?{
                    completionHandler(.success(success))
                }
                else {
                    os_log(.fault, log: StremioApi.oslog, "Casting failed for type: %@, result: %@", String(describing: T.self), String(describing: result))
                }
                Core.removeEventListener(type: callbackType)
            }
        }
    }
}

//useless code cos swiftProtobuf is shit
extension Stremio_Core_Runtime_Event.Error : Error{}
extension Stremio_Core_Runtime_Event.OneOf_Type {
    func get<T>() -> T? {
        switch self {
        case .profilePushedToStorage(let value as T): return value
        case .libraryItemsPushedToStorage(let value as T): return value
        case .streamsPushedToStorage(let value as T): return value
        case .searchHistoryPushedToStorage(let value as T): return value
        case .notificationsPushedToStorage(let value as T): return value
        case .dismissedEventsPushedToStorage(let value as T): return value
        case .userPulledFromApi(let value as T): return value
        case .userPushedToApi(let value as T): return value
        case .addonsPulledFromApi(let value as T): return value
        case .addonsPushedToApi(let value as T): return value
        case .librarySyncWithApiPlanned(let value as T): return value
        case .libraryItemsPushedToApi(let value as T): return value
        case .libraryItemsPulledFromApi(let value as T): return value
        case .userAuthenticated(let value as T): return value
        case .userAddonsLocked(let value as T): return value
        case .userLibraryMissing(let value as T): return value
        case .userLoggedOut(let value as T): return value
        case .sessionDeleted(let value as T): return value
        case .traktAddonFetched(let value as T): return value
        case .traktLoggedOut(let value as T): return value
        case .addonInstalled(let value as T): return value
        case .addonUpgraded(let value as T): return value
        case .addonUninstalled(let value as T): return value
        case .settingsUpdated(let value as T): return value
        case .libraryItemAdded(let value as T): return value
        case .libraryItemRemoved(let value as T): return value
        case .libraryItemRewinded(let value as T): return value
        case .libraryItemNotificationsToggled(let value as T): return value
        case .libraryItemMarkedAsWatched(let value as T): return value
        case .notificationsDismissed(let value as T): return value
        case .playerPlaying(let value as T): return value
        case .playerStopped(let value as T): return value
        case .playerNextVideo(let value as T): return value
        case .playerEnded(let value as T): return value
        case .traktPlaying(let value as T): return value
        case .traktPaused(let value as T): return value
        case .magnetParsed(let value as T): return value
        case .torrentParsed(let value as T): return value
        case .playingOnDevice(let value as T): return value
        case .error(let value as T): return value
        default: return nil
        }
    }
}
