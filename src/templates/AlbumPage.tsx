import { graphql } from 'gatsby'
import * as React from 'react'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { Album } from '../types'

type Props = {
    data: {
        album: Album
    }
}

function seoImage(album: Album): string | null {
    const featuredPhotos = album.photos.filter((photo) => photo.featured)
    const featuredPhoto = featuredPhotos[0]
    if (featuredPhoto) {
        return featuredPhoto.localFile.publicURL
    }

    const otherPhotos = album.photos.filter((photo) => !photo.featured)
    const otherPhoto = otherPhotos[0]
    if (otherPhoto) {
        return otherPhoto.localFile.publicURL
    }

    return null
}

export default function AlbumPage({ data }: Props) {
    const { album } = data

    const photosAndAlbums = React.useMemo(
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
            description="Album"
            image={seoImage(album)}
        >
            <h2 className="pageTitle pb-4">{album.title}</h2>
            {album.description != null && (
                <p className="pb-8">{album.description}</p>
            )}
            <PhotoGrid photosAndAlbums={photosAndAlbums} className="mb-8" />
        </Page>
    )
}

export const pageQuery = graphql`
    query AlbumPageQuery($id: String!) {
        album(id: { eq: $id }) {
            year
            uid
            title
            photos {
                id
                description
                featured
                url
                tags
                localFile {
                    childImageSharp {
                        gatsbyImageData
                        original {
                            width
                            height
                        }
                    }
                }
            }
            date
            description
        }
    }
`
