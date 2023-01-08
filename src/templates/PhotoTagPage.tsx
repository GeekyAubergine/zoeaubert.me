import * as React from 'react'
import { ALBUMS_AND_PHOTOS_BY_TAG } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { usePhotoNodeData } from '../utils'

type Props = {
    pageContext: {
        tag: string
    }
}

export default function AlbumPage({ pageContext }: Props) {
    const { tag } = pageContext

    const photoNodeData = usePhotoNodeData()

    const photos = ALBUMS_AND_PHOTOS_BY_TAG[tag]

    return (
        <Page title="Photos">
            <h2 className="pageTitle">#{tag}</h2>
            <PhotoGrid
                photos={photos}
                photoNodeData={photoNodeData}
                className="mb-8"
            />
        </Page>
    )
}
