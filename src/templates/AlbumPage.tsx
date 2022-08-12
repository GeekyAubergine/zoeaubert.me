import * as React from 'react'
import { Album, ALBUMS_BY_UUID, Photo as PhotoType } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { usePhotoViewer } from '../components/ui/PhotoViewer'

type Props = {
    pageContext: {
        uuid: string
    }
}

export default function AlbumPage({ pageContext }: Props) {
    const { uuid } = pageContext

    const album = ALBUMS_BY_UUID[uuid]

    const { onPhotoClick, Component: PhotoViewerComponent } = usePhotoViewer({
        photos: album.photos,
    })

    const onClickCallback = React.useCallback(
        (photo: PhotoType) => {
            onPhotoClick(photo)
        },
        [onPhotoClick],
    )

    return (
        <Page title="Photos">
            <h2 className="pageTitle">
                {album.title}
            </h2>
            {album.description != null && (
                <p className="pb-2 font-bold sm:pb-6">{album.description}</p>
            )}
            <PhotoGrid
                photos={album.photos}
                className="mb-8"
                onClick={onClickCallback}
            />
            {PhotoViewerComponent}
        </Page>
    )
}
