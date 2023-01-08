import * as React from 'react'
import { ALBUMS_BY_DATE, PhotoWithAlbum } from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import PhotoGrid from '../../components/ui/PhotoGrid'
import { usePhotoNodeData } from '../../utils'

export default function AllPhotos() {
    const photoNodeData = usePhotoNodeData()

    const allPhotos = React.useMemo(
        () =>
            ALBUMS_BY_DATE.reduce<PhotoWithAlbum[]>((acc, album) => {
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
            <PhotoGrid
                photos={allPhotos}
                photoNodeData={photoNodeData}
                className="mb-8"
            />
        </Page>
    )
}
