import { graphql } from 'gatsby'
import * as React from 'react'
import SEO from '../components/Seo'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { Album } from '../types'
import { isPhotoLandscape } from '../utils'

type Props = {
    data: {
        album: Album
    }
}

function seoImage(album: Album): string | null {
    const featuredPhotos = album.photos.filter((photo) => photo.featured)

    const featuredLandscapes = featuredPhotos.filter(isPhotoLandscape)

    const featuredLandscape = featuredLandscapes[0]

    if (featuredLandscape) {
        return featuredLandscape.localFile.publicURL
    }

    const featuredPhoto = featuredPhotos[0]
    if (featuredPhoto) {
        return featuredPhoto.localFile.publicURL
    }

    const otherPhotos = album.photos.filter((photo) => !photo.featured)

    const otherLandscapes = otherPhotos.filter(isPhotoLandscape)

    const otherLandscape = otherLandscapes[0]

    if (otherLandscape) {
        return otherLandscape.localFile.publicURL
    }

    const otherPhoto = otherPhotos[0]
    if (otherPhoto) {
        return otherPhoto.localFile.publicURL
    }

    return null
}

export default function AlbumPage({ data }: Props) {
    const { album } = data

    return (
        <Page widthControlled={false}>
            <div className="width-control mx-auto">
                <div className="pb-4">
                    <h2 className="pageTitle">{album.title}</h2>
                    <time className="text secondary" dateTime={album.date}>
                        {album.date}
                    </time>
                </div>
                {album.description != null && (
                    <p className="pb-8">{album.description}</p>
                )}
            </div>
            <PhotoGrid photos={album.photos} />
        </Page>
    )
}

export function Head({ data }: Props) {
    const { album } = data

    return (
        <SEO
            title={`${album.title}`}
            description={album.title}
            image={seoImage(album)}
        />
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
                alt
                featured
                url
                tags
                photoIndex
                localFile {
                    childImageSharp {
                        gatsbyImageData
                        original {
                            width
                            height
                        }
                    }
                    publicURL
                }
                album {
                    title
                    date
                }
            }
            date
            description
        }
    }
`
