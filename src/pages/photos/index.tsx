import * as React from 'react'
import {
    Album,
    Photo as PhotoType,
    albumToSlug,
    ALBUMS_BY_DATE,
} from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import Photo from '../../components/ui/Photo'
import { DateTime } from 'luxon'
import PhotoGrid from '../../components/ui/PhotoGrid'
import { Link } from 'gatsby'

const PHOTOS_FOR_ALBUM_COVER = 4
const MAX_FEATURED_PHOTOS = 9

function renderPhoto(photo: PhotoType) {
    return <Photo photo={photo} key={photo.url} className="!rounded-none" />
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
            <h4 className="!text-sm underline">{album.title}</h4>
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

    console.log({ albumsByYear })

    const renderYear = React.useCallback(
        (year) => {
            const albums = albumsByYear[year]
            return (
                <div key={year}>
                    <h3 className='text-xl'>{year}</h3>
                    <div className="grid gap-x-2 gap-y-2 grid-cols-1 sm:grid-cols-2 md:grid-cols-3 mb-8">
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

    return (
        <Page title="Photos">
            <div className="flex justify-between items-baseline">
                <h2 className="text-2xl font-bold">Photos</h2>
                <Link to="/photos/all" className='link'>All Photos</Link>
            </div>
            {featuredPhotos.length > 0 && (
                <PhotoGrid photos={featuredPhotos} className="mb-8" />
            )}
            {years.map(renderYear)}
        </Page>
    )
}
