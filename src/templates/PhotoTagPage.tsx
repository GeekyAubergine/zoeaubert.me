import * as React from 'react'
import { Album, ALBUMS, Photo as PhotoType } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { usePhotoViewer } from '../components/ui/PhotoViewer'

type Props = {
    pageContext: {
        tag: string
    }
}

export default function AlbumPage({ pageContext }: Props) {
    const { tag } = pageContext

    const photos = React.useMemo(
        () =>
            ALBUMS.reduce(
                (acc: PhotoType[], album) =>
                    acc.concat(
                        album.photos.filter((photo) =>
                            photo.tags.includes(tag),
                        ),
                    ),
                [],
            ),
        [],
    )

    const { onPhotoClick, Component: PhotoViewerComponent } = usePhotoViewer({
        photos,
    })

    const onClickCallback = React.useCallback(
        (photo: PhotoType) => {
            onPhotoClick(photo)
        },
        [onPhotoClick],
    )

    return (
        <Page title="Photos">
            <h2 className="text-2xl pt-12 mb-2 font-bold sm:pt-8">
                #{tag}
            </h2>
            <PhotoGrid
                photos={photos}
                className="mb-8"
                onClick={onClickCallback}
            />
            {PhotoViewerComponent}
        </Page>
    )
}
