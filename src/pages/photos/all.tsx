import * as React from 'react'
import { ALBUMS_BY_DATE, PhotoAndAlbum } from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import PhotoGrid from '../../components/ui/PhotoGrid'

export default function AllPhotos() {
    const allPhotos = React.useMemo(
        () =>
            ALBUMS_BY_DATE.reduce<PhotoAndAlbum[]>((acc, album) => {
                return acc.concat(
                    album.photos.map((photo) => ({
                        photo,
                        album,
                    })),
                )
            }, []),
        [],
    )

    return (
        <Page title="Photos">
            <h2 className="pageTitle mb-4">All Photos</h2>
            <PhotoGrid photosAndAlbums={allPhotos} className="mb-8" />
        </Page>
    )
}
