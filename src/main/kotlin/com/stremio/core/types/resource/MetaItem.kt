package com.stremio.core.types.resource

import com.stremio.core.deeplinks.MetaItemDeepLinks
import com.stremio.core.v1.StreamDeepLinks
import pbandk.wkt.Timestamp
import java.util.*

data class MetaItem(
    val id: String,
    val type: String,
    val name: String,
    val poster: String?,
    val background: String?,
    val logo: String?,
    val description: String?,
    val releaseInfo: String?,
    val runtime: String?,
    val released: Date?,
    val posterShape: PosterShape,
    val links: List<Link>,
    val trailerStreams: List<Stream>,
    val videos: List<Video>,
    val behaviorHints: MetaItemBehaviorHints,
    val deepLinks: MetaItemDeepLinks,
) {
    fun toProtobuf(): com.stremio.core.v1.MetaItem {
        return com.stremio.core.v1.MetaItem(
            id = id,
            type = type,
            name = name,
            posterShape = com.stremio.core.v1.PosterShape.fromName(posterShape.toString()),
            poster = poster,
            background = background,
            logo = logo,
            description = description,
            releaseInfo = releaseInfo,
            runtime = runtime,
            released = released?.let {
                Timestamp(seconds = it.time)
            },
            links = links.map {
                com.stremio.core.v1.Link(
                    name = it.name,
                    category = it.category,
                    url = it.url
                )
            },
            trailerStreams = trailerStreams.map {
                com.stremio.core.v1.Stream(
                    source = com.stremio.core.v1.Stream.Source.Youtube(
                        com.stremio.core.v1.Stream.YouTube((it.source as StreamSource.YouTube).ytId)
                    ),
                    name = it.name,
                    description = it.description,
                    thumbnail = it.thumbnail,
                    behaviorHints = com.stremio.core.v1.StreamBehaviorHints(
                        notWebReady = it.behaviorHints.notWebReady,
                        bingeGroup = it.behaviorHints.bingeGroup
                    ),
                    deepLinks = com.stremio.core.v1.StreamDeepLinks(
                        player = it.deepLinks.player,
                        externalPlayer = StreamDeepLinks.ExternalPlayerLink(
                            href = it.deepLinks.externalPlayer.href,
                            download = it.deepLinks.externalPlayer.download
                        )
                    )
                )
            },
            videos = videos.map {
                com.stremio.core.v1.Video(
                    id = it.id,
                    title = it.title,
                    released = it.released?.let { date ->
                        Timestamp(seconds = date.time)
                    },
                    overview = it.overview,
                    thumbnail = it.thumbnail,
                    seriesInfo = it.seriesInfo?.let { info ->
                        com.stremio.core.v1.Video.SeriesInfo(info.season, info.episode)
                    }
                )
            },
            behaviorHints = com.stremio.core.v1.MetaItemBehaviorHints(
                hasScheduledVideos = behaviorHints.hasScheduledVideos,
                defaultVideoId = behaviorHints.defaultVideoId,
                featuredVideoId = behaviorHints.featuredVideoId
            ),
            deepLinks = com.stremio.core.v1.MetaItemDeepLinks(
                metaDetailsVideos = deepLinks.metaDetailsVideos,
                metaDetailsStreams = deepLinks.metaDetailsStreams
            ),
        )
    }
}
