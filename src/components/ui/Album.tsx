import React, { useCallback } from 'react'
import { Link } from 'gatsby'
import Photo from '../../components/ui/Photo'
import { Album as AlbumType, Photo as PhotoType } from '../../types'
import { albumToSlug, isPhotoLandscape, isPhotoPortrait } from '../../utils'

type Props = {
    album: AlbumType
}

function renderPhoto(photo: PhotoType) {
    return (
        <Photo
            photo={photo}
            key={photo.url}
            className="object-cover sm:max-h-[12rem] "
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
            <div className="grid gap-x-[1px] gap-y-[1px] grid-cols-2 rounded-md overflow-hidden">
                {children}
            </div>
            <h4 className="link text-sm text-center my-1">{album.title}</h4>
        </Link>
    )
}

export default function Album({ album }: Props) {
    const renderAlbumPhotos = useCallback(
        (photo: PhotoType) => {
            if (!album) {
                return null
            }
            return renderPhoto(photo)
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
        isPhotoLandscape(featuredPhotos[0])
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
        isPhotoLandscape(featuredPhotos[0])
    ) {
        return (
            <AlbumWrapper album={album} single>
                {renderAlbumPhotos(otherPhotos[0])}
            </AlbumWrapper>
        )
    }

    // If portrait show the first two available portrait photos

    const featuredPortraitPhotos = featuredPhotos.filter((photo) =>
        isPhotoPortrait(photo),
    )

    const otherPortraitPhotos = otherPhotos.filter((photo) =>
        isPhotoPortrait(photo),
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
