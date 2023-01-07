import * as React from 'react'
import {
    Album,
    Photo as PhotoType,
    PhotoLegacy as PhotoLegacyType,
    albumToSlug,
    ALBUMS_BY_DATE,
    AlbumLegacy,
} from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import { DateTime } from 'luxon'
import PhotoGrid from '../../components/ui/PhotoGrid'
import { Link } from 'gatsby'
import { usePhotoViewer } from '../../components/ui/PhotoViewer'
import Photo from '../../components/ui/Photo'

const PHOTOS_FOR_ALBUM_COVER = 2
const MAX_FEATURED_PHOTOS = 9

function renderPhoto(photo: PhotoType) {
    return (
        <Photo
            photo={photo}
            key={photo.path}
            className="!rounded-none max-h-[6rem] object-cover"
        />
    )
}
function renderAlbum(album: Album) {
    const featuredPhotos = album.photos.filter((photo) => photo.featured)
    const otherPhotos = album.photos.filter((photo) => !photo.featured)

    const photosForCover = featuredPhotos
        .slice(0, PHOTOS_FOR_ALBUM_COVER)
        .concat(
            otherPhotos.slice(
                0,
                PHOTOS_FOR_ALBUM_COVER - featuredPhotos.length,
            ),
        )

    return (
        <Link className="cursor-pointer" to={albumToSlug(album)}>
            <div className="grid gap-x-[1px] gap-y-[1px] grid-cols-2 bg-black rounded-md overflow-hidden">
                {photosForCover.map(renderPhoto)}
            </div>
            <h4 className="!text-sm underline ml-2">{album.title}</h4>
        </Link>
    )
}

export default function IndexPage() {
    const albumsByYear = React.useMemo(
        () =>
            ALBUMS_BY_DATE.reduce((acc, album) => {
                const year = DateTime.fromISO(album.date).year
                return {
                    ...acc,
                    [year]: [...(acc[year] || []), album],
                }
            }, {}),
        [],
    )

    const years = React.useMemo(
        () => Object.keys(albumsByYear).sort().reverse(),
        [albumsByYear],
    )

    const renderYear = React.useCallback(
        (year) => {
            const albums = albumsByYear[year]

            return (
                <div key={year} className="my-2">
                    <h3 className="">{year}</h3>
                    <div className="grid gap-x-2 gap-y-2 grid-cols-2 sm:grid-cols-2 md:grid-cols-3 mb-8">
                        {albums.map(renderAlbum)}
                    </div>
                </div>
            )
        },
        [albumsByYear],
    )

    const featuredPhotos: PhotoType[] = React.useMemo(() => {
        return ALBUMS_BY_DATE.reduce(
            (acc: PhotoType[], album) =>
                acc.concat(album.photos.filter((photo) => photo.featured)),
            [],
        ).slice(0, MAX_FEATURED_PHOTOS)
    }, [])

    const { onPhotoClick, Component: PhotoViewerComponent } = usePhotoViewer({
        photos: featuredPhotos,
    })

    const onClickCallback = React.useCallback(
        (photo: PhotoType) => {
            onPhotoClick(photo)
        },
        [onPhotoClick],
    )

    return (
        <Page title="Photos">
            <div className="flex justify-between items-baseline">
                <h2 className='pageTitle'>Photos</h2>
                <div>
                    <Link to="/photos/tags" className="link mr-4">
                        Tags
                    </Link>
                    <Link to="/photos/all" className="link">
                        All Photos
                    </Link>
                </div>
            </div>
            {featuredPhotos.length > 0 && (
                <PhotoGrid
                    photos={featuredPhotos}
                    className="mb-8"
                    onClick={onClickCallback}
                />
            )}
            {years.map(renderYear)}
            {PhotoViewerComponent}
        </Page>
    )
}
