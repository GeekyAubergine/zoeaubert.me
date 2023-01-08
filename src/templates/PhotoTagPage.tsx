import * as React from 'react'
import { ALBUMS_AND_PHOTOS_BY_TAG } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'

type Props = {
    pageContext: {
        tag: string
    }
}

export default function AlbumPage({ pageContext }: Props) {
    const { tag } = pageContext

    const photosAndAlbums = ALBUMS_AND_PHOTOS_BY_TAG[tag]

    return (
        <Page title="Photos">
            <h2 className="pageTitle">#{tag}</h2>
            <PhotoGrid photosAndAlbums={photosAndAlbums} className="mb-8" />
        </Page>
    )
}
