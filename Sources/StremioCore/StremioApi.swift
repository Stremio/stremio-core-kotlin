//
//  StremioApi.swift
//  Stremio
//
//  Created by Alvin on 29.01.24.
//

import Foundation
import SwiftProtobuf
import os.log

public class StremioApi {
    internal static let oslog = OSLog(subsystem: "com.stremio.core.StremioApi", category: "Wrapper")

    public static func SetLoadRange(field: Stremio_Core_Runtime_Field?, start: UInt32, end: UInt32) {
        var action = Stremio_Core_Runtime_Action()
        if field == .board || field == .discover{
            action.load.catalogsWithExtra = Stremio_Core_Models_CatalogsWithExtra.Selected()
            Core.dispatch(action: action, field: field)
        }
        
        action = Stremio_Core_Runtime_Action()
        action.catalogsWithExtra.loadRange.start = start
        action.catalogsWithExtra.loadRange.end = end
        Core.dispatch(action: action, field: field)
    }
    
    //MARK: - set filters
    public static func LoadLibraryWithFilters(_ request:  Stremio_Core_Models_LibraryWithFilters.LibraryRequest){
        var action = Stremio_Core_Runtime_Action()
        action.load.libraryWithFilters.request = request
        Core.dispatch(action: action, field: .library)
    }

    public static func LoadDiscoverCatalog(_ request: Stremio_Core_Types_ResourceRequest){
        var action = Stremio_Core_Runtime_Action()
        action.load.catalogWithFilters.request = request
        Core.dispatch(action: action, field: .discover)
    }
    
    public static func LoadAddons(_ request: Stremio_Core_Types_ResourceRequest){
        var action = Stremio_Core_Runtime_Action()
        action.load.addonsWithFilters.request = request
        Core.dispatch(action: action, field: .addons)
    }
    
    //MARK: - load states
    public static func LoadBoard() -> Stremio_Core_Models_CatalogsWithExtra? {
        if let myMessage: Stremio_Core_Models_CatalogsWithExtra = Core.getState(.board) {
           return myMessage
        }
        return nil
    }
    
    public static func LoadContinueWatchingPreview() -> Stremio_Core_Models_ContinueWatchingPreview? {
        if let myMessage: Stremio_Core_Models_ContinueWatchingPreview = Core.getState(.continueWatchingPreview) {
           return myMessage
        }
        return nil
    }
    
    public static func LoadDiscover() -> Stremio_Core_Models_CatalogWithFilters? {
        if let myMessage: Stremio_Core_Models_CatalogWithFilters = Core.getState(.discover) {
           return myMessage
        }
        return nil
    }
    
    public static func LoadLibrary() -> Stremio_Core_Models_LibraryWithFilters? {
        if let myMessage: Stremio_Core_Models_LibraryWithFilters = Core.getState(.library) {
           return myMessage
        }
        return nil
    }
    
    public static func LoadNextDiscoverCatalogPage() {
        var action = Stremio_Core_Runtime_Action()
        action.catalogWithFilters.loadNextPage = Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .discover)
    }

    public static func LoadAddons() -> Stremio_Core_Models_AddonsWithFilters? {
        if let myMessage: Stremio_Core_Models_AddonsWithFilters = Core.getState(.addons) {
           return myMessage
        }
        return nil
    }
    
    public static func AddonDetails(transportURL: String,
                                    completionHandler: @escaping ((Stremio_Core_Models_AddonDetails) -> Void)) {
        Core.addEventListener(type: .addonDetails, { _ in
            if let myMessage: Stremio_Core_Models_AddonDetails = Core.getState(.addonDetails) {
                completionHandler(myMessage)
                if case .loading(_:) = myMessage.remoteAddon.content{}
                else {
                    Core.removeEventListener(type: .addonDetails)
                    StremioApi.Unload(field: .addonDetails)
                }
            }
            else {
                Core.removeEventListener(type: .addonDetails)
                StremioApi.Unload(field: .addonDetails)
            }
        })
        StremioApi.AddonItemLoad(transportURL: transportURL)
    }

    public static func LoadCtx() -> Stremio_Core_Models_Ctx? {
        if let myMessage: Stremio_Core_Models_Ctx = Core.getState(.ctx) {
           return myMessage
        }
        return nil
    }
    
    public static func LoadPlayer() -> Stremio_Core_Models_Player? {
        if let myMessage: Stremio_Core_Models_Player = Core.getState(.player) {
           return myMessage
        }
        return nil
    }
    
    //MARK: - For search
    
    public static func Search(_ searchString: String) {
        var action = Stremio_Core_Runtime_Action()
        SetLoadRange(field: .search, start: 0, end: 6)

        var searchRequest = Stremio_Core_Types_ExtraValue()
        searchRequest.name = "search"; searchRequest.value = searchString

        action = Stremio_Core_Runtime_Action()
        action.load.catalogsWithExtra.extra = [searchRequest]
        Core.dispatch(action: action, field: .search)
        
        SetLoadRange(field: .search, start: 0, end: 1)
    }
    
    public static func getSearchResults() -> Stremio_Core_Models_CatalogsWithExtra? {
        if let myMessage: Stremio_Core_Models_CatalogsWithExtra = Core.getState(.search) {
           return myMessage
        }
        return nil
    }
    
    // MARK: - Function to load MetaIteam detailed
    public static func MetaItemLoad(metaItem: MetaItem) {
        var action = Stremio_Core_Runtime_Action()
        action.load.metaDetails.metaPath.resource = "meta"
        action.load.metaDetails.metaPath.type = metaItem.type
        action.load.metaDetails.metaPath.id =  metaItem.type == "series" ? String(metaItem.id.prefix(while: { $0 != ":" })) : metaItem.id
        action.load.metaDetails.streamPath.resource = "stream"
        action.load.metaDetails.streamPath.type = metaItem.type
        action.load.metaDetails.streamPath.id = metaItem.id
        action.load.metaDetails.guessStreamPath = true
        Core.dispatch(action: action, field: .metaDetails)
    }
    
    public static func MetaItemGet() -> Stremio_Core_Models_MetaDetails?{
        if let myMessage: Stremio_Core_Models_MetaDetails = Core.getState(.metaDetails) {
           return myMessage
        }
        return nil
    }
    
    public static func SyncAddons() {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.pullAddonsFromApi = SwiftProtobuf.Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .ctx)
    }

    public static func SyncLibray() {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.syncLibraryWithApi = SwiftProtobuf.Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .ctx)
    }
    
    public static func PullUser() {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.pullUserFromApi = SwiftProtobuf.Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .ctx)
    }
    
    public static func PullNotifications() {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.pullNotifications = SwiftProtobuf.Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .ctx)
    }

    public static func PullEvents() {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.getEvents = SwiftProtobuf.Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .ctx)
    }
    
    public static func Unload(field: Stremio_Core_Runtime_Field?) {
        var action = Stremio_Core_Runtime_Action()
        action.unload = Stremio_Core_Runtime_Action.ActionUnload()
        Core.dispatch(action: action, field: field)
    }
    //MARK: -- VideoPlayer
    public static func PlayerItemLoad(urlPath: [String]) {
        if urlPath.isEmpty {return}
        guard let stream = Core.decodeStreamData(streamData: urlPath[1]) else {return}
        
        var action = Stremio_Core_Runtime_Action()
        action.load.player.stream = stream
        //If url contains info about meta then load it also
        if urlPath.indices.contains(5) {
            let addonURL = urlPath[2]
            let metaURL = urlPath[3]
            let contentType = urlPath[4]
            let contentID = urlPath[5]
            let streamID = urlPath.indices.contains(6) ?  urlPath[6] : contentID
            
            action.load.player.streamRequest.base = addonURL
            action.load.player.streamRequest.path.resource = "stream"
            action.load.player.streamRequest.path.id = streamID
            action.load.player.streamRequest.path.type = contentType
            
            action.load.player.metaRequest.base = metaURL
            action.load.player.metaRequest.path.resource = "meta"
            action.load.player.metaRequest.path.id = contentID
            action.load.player.metaRequest.path.type = contentType
            
            action.load.player.subtitlesPath.resource = "subtitles"
            action.load.player.subtitlesPath.id = streamID
            action.load.player.subtitlesPath.type = contentType
        }
        Core.dispatch(action: action, field: .player)
    }
    
    public static func PlayerTimeChanged(duration: Double, time: Double){
        var action = Stremio_Core_Runtime_Action()
        #if targetEnvironment(macCatalyst)
        action.player.timeChanged.device = "AppleMacOS"
        #elseif os(iOS)
        action.player.timeChanged.device = "AppleiOS"
        #elseif os(tvOS)
        action.player.timeChanged.device = "AppletvOS"
        #endif
        action.player.timeChanged.duration = UInt64(duration) * 1000
        action.player.timeChanged.time =  UInt64(time) * 1000
        Core.dispatch(action: action, field: .player)
    }
    
    public static func PlayerSetStatus(isPaused: Bool){
        var action = Stremio_Core_Runtime_Action()
        action.player.pausedChanged = isPaused
        Core.dispatch(action: action, field: .player)
    }
    
    public static func PlayerVideoParmChanged(){
        var action = Stremio_Core_Runtime_Action()
        action.player.videoParamsChanged.clearFilename()
        action.player.videoParamsChanged.clearHash()
        action.player.videoParamsChanged.clearSize()
        Core.dispatch(action: action, field: .player)
    }
    
    
    //MARK: -- Addons
    public static func AddonItemLoad(transportURL: String) {
        var action = Stremio_Core_Runtime_Action()
        action.load.addonDetails.transportURL = transportURL
        Core.dispatch(action: action, field: .addonDetails)
    }
    
    public static func UninstallAddon(descriptor: Stremio_Core_Types_Descriptor, 
                                      completionHandler: ((Result<Stremio_Core_Runtime_Event.AddonUninstalled, Stremio_Core_Runtime_Event.Error> ) -> Void )? = nil) {
        handleEvent(callbackType: CallbackType.addonUninstalled, completionHandler: completionHandler)
        var action = Stremio_Core_Runtime_Action()
        action.ctx.uninstallAddon = descriptor
        Core.dispatch(action: action)
    }
    
    public static func InstallAddon(descriptor: Stremio_Core_Types_Descriptor,
                                    completionHandler: ((Result<Stremio_Core_Runtime_Event.AddonInstalled, Stremio_Core_Runtime_Event.Error>) -> Void)? = nil) {
        handleEvent(callbackType: CallbackType.addonInstalled, completionHandler: completionHandler)
        var action = Stremio_Core_Runtime_Action()
        action.ctx.installAddon = descriptor
        Core.dispatch(action: action)
    }
    
    public static func UpgradeAddon(descriptor: Stremio_Core_Types_Descriptor,
                                    completionHandler: ((Result<Stremio_Core_Runtime_Event.AddonUpgraded, Stremio_Core_Runtime_Event.Error> ) -> Void)? = nil) {
        handleEvent(callbackType: CallbackType.addonUpgraded, completionHandler: completionHandler)
        var action = Stremio_Core_Runtime_Action()
        action.ctx.upgradeAddon = descriptor
        Core.dispatch(action: action)
    }
    
    public static func UpdateSettings(settings: Stremio_Core_Types_Profile.Settings,
                                      completionHandler: ((Result<Stremio_Core_Runtime_Event.SettingsUpdated, Stremio_Core_Runtime_Event.Error>) -> Void)? = nil) {
        handleEvent(callbackType: CallbackType.settingsUpdated, completionHandler: completionHandler)
        var action = Stremio_Core_Runtime_Action()
        action.ctx.updateSettings = settings
        Core.dispatch(action: action)
    }

    //MARK: - Adding, removing Library
    
    public static func AddToLibrary(metaPreview: Stremio_Core_Types_MetaItemPreview) {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.addToLibrary = metaPreview
        Core.dispatch(action: action)
    }
    
    public static func RemoveFromLibrary(metaID: String) {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.removeFromLibrary = metaID
        Core.dispatch(action: action)
    }
    
    public static func RewindLibraryItem(metaID: String?){
        guard let metaID = metaID else { return }
        var action = Stremio_Core_Runtime_Action()
        action.ctx.rewindLibraryItem = metaID
        Core.dispatch(action: action)
    }
    
    public static func MarkAsWatched(video: Stremio_Core_Types_Video, watched: Bool){
        var state = Stremio_Core_Runtime_ActionMetaDetails.VideoState()
        state.video = video
        state.isWatched = watched
        var action = Stremio_Core_Runtime_Action()
        action.metaDetails.markVideoAsWatched = state
        Core.dispatch(action: action, field: .metaDetails)
    }
    
    //MARK: - For account related functions

    public static func LoginWithToken(token: String) {
        //TODO: Implimnet logining with token
    }
    
    public static func Login(email: String, password: String,
                             completionHandler: ((Result<Stremio_Core_Runtime_Event.UserAuthenticated, Stremio_Core_Runtime_Event.Error>) -> Void)? = nil) {
        handleEvent(callbackType: CallbackType.userAuthenticated, completionHandler: completionHandler)
        var action = Stremio_Core_Runtime_Action()
        action.ctx.authenticate.login.email = email
        action.ctx.authenticate.login.password = password
        action.ctx.authenticate.login.facebook = false
        Core.dispatch(action: action, field: .ctx)
    }
    
    public static func Logout() {
        var action = Stremio_Core_Runtime_Action()
        action.ctx.logout = SwiftProtobuf.Google_Protobuf_Empty()
        Core.dispatch(action: action, field: .ctx)
    }
}

public protocol MetaItem {
    var id: String { get }
    var type: String { get }
    var name: String { get }
}

extension Stremio_Core_Types_MetaItem: MetaItem {}

extension Stremio_Core_Types_MetaItemPreview: MetaItem {}
