import * as React from 'react'
import { Photo as PhotoType, ALBUMS_BY_DATE } from '../../../res/photos'
import { Page } from '../../components/ui/Page'
import PhotoGrid from '../../components/ui/PhotoGrid'
import { usePhotoViewer } from '../../components/ui/PhotoViewer'

export default function AllPhotos() {
    const allPhotos: PhotoType[] = React.useMemo(
        () =>
            ALBUMS_BY_DATE.reduce(
                (acc: PhotoType[], album) => acc.concat(album.photos),
                [],
            ),
        [],
    )

    const { onPhotoClick, Component: PhotoViewerComponent } = usePhotoViewer({
        photos: allPhotos,
    })

    const onClickCallback = React.useCallback(
        (photo: PhotoType) => {
            onPhotoClick(photo)
        },
        [onPhotoClick],
    )

    return (
        <Page title="Photos">
            <h2 className="text-xl mb-2 font-bold sm:pt-8">
                All Photos
            </h2>
            <PhotoGrid
                photos={allPhotos}
                className="mb-8"
                onClick={onClickCallback}
            />
            {PhotoViewerComponent}
        </Page>
    )
}
