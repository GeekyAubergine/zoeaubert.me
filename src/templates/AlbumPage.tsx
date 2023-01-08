import * as React from 'react'
import { ALBUMS_BY_UUID, Album, PhotoNodeData } from '../../res/photos'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { usePhotoNodeData } from '../utils'

type Props = {
    pageContext: {
        uuid: string
    }
}

function seoImage(album: Album, photoNodeData: PhotoNodeData[]): string | null {
    const featuredPhotos = album.photos.filter((photo) => photo.featured)
    const featuredPhoto = featuredPhotos[0]
    if (featuredPhoto) {
        const cleanedPath = featuredPhoto.path.replace(/^\//, '')

        const imageNode = photoNodeData.find(
            (node) => node.relativePath === cleanedPath,
        )

        if (imageNode) {
            return imageNode.publicURL
        }
    }

    const otherPhotos = album.photos.filter((photo) => !photo.featured)
    const otherPhoto = otherPhotos[0]
    if (otherPhoto) {
        const cleanedPath = otherPhoto.path.replace(/^\//, '')

        const imageNode = photoNodeData.find(
            (node) => node.relativePath === cleanedPath,
        )

        if (imageNode) {
            return imageNode.publicURL
        }
    }

    return null
}

export default function AlbumPage({ pageContext }: Props) {
    const { uuid } = pageContext

    const album = ALBUMS_BY_UUID[uuid]

    const photoNodeData = usePhotoNodeData()

    const photos = React.useMemo(
        () =>
            album.photos.map((photo) => ({
                photo,
                album,
            })),
        [album],
    )

    return (
        <Page
            title={`${album.title} | Photos`}
            description={album.description}
            image={seoImage(album, photoNodeData)}
        >
            <h2 className="pageTitle pb-4">{album.title}</h2>
            {album.description != null && (
                <p className="pb-8">{album.description}</p>
            )}
            <PhotoGrid
                photos={photos}
                photoNodeData={photoNodeData}
                className="mb-8"
            />
        </Page>
    )
}
