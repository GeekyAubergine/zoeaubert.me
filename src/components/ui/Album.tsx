import React, { useCallback, useMemo } from 'react'
import {
    Album as AlbumType,
    Photo as PhotoType,
    albumToSlug,
    ALBUMS_BY_YEAR,
    ALBUMS_BY_UUID,
} from '../../../res/photos'
import { Link } from 'gatsby'
import Photo from '../../components/ui/Photo'

const PHOTOS_FOR_ALBUM_COVER = 2

type Props = {
    uuid: string
}

function renderPhoto(photo: PhotoType, album: AlbumType) {
    const photoAndAlbum = useMemo(() => ({ photo, album }), [photo, album])

    return (
        <Photo
            photoAndAlbum={photoAndAlbum}
            key={photo.path}
            className="!rounded-none object-cover sm:max-h-[12rem]"
            disableLink
        />
    )
}

function AlbumWrapper({
    album,
    single,
    children,
}: {
    album: AlbumType
    single?: boolean
    children: React.ReactNode
}) {
    if (single) {
        return (
            <Link className="cursor-pointer my-2" to={albumToSlug(album)}>
                <div />
                <div className="bg-black rounded-md overflow-hidden">
                    {children}
                </div>
                <h4 className="link text-sm text-center my-1">{album.title}</h4>
            </Link>
        )
    }

    return (
        <Link className="cursor-pointer my-2" to={albumToSlug(album)}>
            <div className="grid gap-x-[1px] gap-y-[1px] grid-cols-2 bg-black rounded-md overflow-hidden">
                {children}
            </div>
            <h4 className="link text-sm text-center my-1">{album.title}</h4>
        </Link>
    )
}

export default function Album({ uuid }: Props) {
    const album = ALBUMS_BY_UUID[uuid]

    const renderAlbumPhotos = useCallback(
        (photo) => {
            if (!album) {
                return null
            }
            return renderPhoto(photo, album)
        },
        [album],
    )

    if (!album) {
        return null
    }

    const featuredPhotos = album.photos.filter((photo) => photo.featured)

    // If landscape just show the first photo

    if (
        featuredPhotos.length > 0 &&
        featuredPhotos[0] &&
        featuredPhotos[0].orientation === 'landscape'
    ) {
        return (
            <AlbumWrapper album={album} single>
                {renderAlbumPhotos(featuredPhotos[0])}
            </AlbumWrapper>
        )
    }

    const otherPhotos = album.photos.filter((photo) => !photo.featured)

    if (
        otherPhotos.length > 0 &&
        otherPhotos[0] &&
        otherPhotos[0].orientation === 'landscape'
    ) {
        return (
            <AlbumWrapper album={album} single>
                {renderAlbumPhotos(otherPhotos[0])}
            </AlbumWrapper>
        )
    }

    // If portrait show the first two available portrait photos

    const featuredPortraitPhotos = featuredPhotos.filter(
        (photo) => photo.orientation === 'portrait',
    )

    const otherPortraitPhotos = otherPhotos.filter(
        (photo) => photo.orientation === 'portrait',
    )

    if (featuredPortraitPhotos.length >= 2) {
        return (
            <AlbumWrapper album={album}>
                {featuredPortraitPhotos.slice(0, 2).map(renderAlbumPhotos)}
            </AlbumWrapper>
        )
    }

    if (
        featuredPortraitPhotos.length === 1 &&
        otherPortraitPhotos.length >= 1
    ) {
        return (
            <AlbumWrapper album={album}>
                {featuredPortraitPhotos.map(renderAlbumPhotos)}
                {otherPortraitPhotos.slice(0, 1).map(renderAlbumPhotos)}
            </AlbumWrapper>
        )
    }

    if (otherPortraitPhotos.length >= 2) {
        return (
            <AlbumWrapper album={album}>
                {otherPortraitPhotos.slice(0, 2).map(renderAlbumPhotos)}
            </AlbumWrapper>
        )
    }

    // If portrait but only one photo available, show the first photo and any other photo

    if (featuredPortraitPhotos.length === 1) {
        return (
            <AlbumWrapper album={album}>
                {featuredPortraitPhotos.map(renderAlbumPhotos)}
                {otherPhotos.slice(0, 1).map(renderAlbumPhotos)}
            </AlbumWrapper>
        )
    }

    return (
        <AlbumWrapper album={album}>
            {otherPhotos.slice(0, 2).map(renderAlbumPhotos)}
        </AlbumWrapper>
    )
}
