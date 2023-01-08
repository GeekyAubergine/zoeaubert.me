import * as React from 'react'
import { ALBUMS_BY_UUID } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'

type Props = {
    pageContext: {
        uuid: string
    }
}

export default function AlbumPage({ pageContext }: Props) {
    const { uuid } = pageContext

    const album = ALBUMS_BY_UUID[uuid]

    const photosAndAlbums = React.useMemo(
        () =>
            album.photos.map((photo) => ({
                photo,
                album,
            })),
        [album],
    )

    return (
        <Page title={`${album.title} | Photos`} description={album.description}>
            <h2 className="pageTitle pb-4">{album.title}</h2>
            {album.description != null && (
                <p className="pb-8">{album.description}</p>
            )}
            <PhotoGrid photosAndAlbums={photosAndAlbums} className="mb-8" />
        </Page>
    )
}
