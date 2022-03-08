package com.stremio.core.runtime.msg

sealed class ActionLoad {
    data class CatalogWithFilters(val args: com.stremio.core.models.CatalogWithFilters.Selected) :
        ActionLoad()

    data class CatalogsWithExtra(val args: com.stremio.core.models.CatalogsWithExtra.Selected) :
        ActionLoad()

    data class LibraryWithFilters(val args: com.stremio.core.models.LibraryWithFilters.Selected) :
        ActionLoad()

    data class MetaDetails(val args: com.stremio.core.models.MetaDetails.Selected) :
        ActionLoad()

    class Link : ActionLoad()

//    data class Player(val args: com.stremio.core.models.Player.Selected) :
//        ActionLoad()
}
