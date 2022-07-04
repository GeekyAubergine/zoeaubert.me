import * as React from 'react'
import { Album } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'

type Props = {
    pageContext: {
        album: Album
    }
}

export default function AlbumPage({ pageContext }: Props) {
    const { album } = pageContext
    return (
        <Page title="Photos">
            <h2 className="text-2xl pt-12 mb-2 font-bold sm:pt-8">
                {album.title}
            </h2>
            {album.description != null && (
                <p className="pb-2 font-bold sm:pb-6">
                    {album.description}
                </p>
            )}
            <PhotoGrid photos={album.photos} className="mb-8" />
        </Page>
    )
}
