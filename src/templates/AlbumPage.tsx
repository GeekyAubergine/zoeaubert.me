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
    console.log({ featuredPhoto })
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

    console.log({ i: seoImage(album) })

    return (
        <Page
            title={`${album.title} | Photos`}
            description={album.title}
            image={seoImage(album)}
            widthControlled={false}
        >
            <div className="width-control mx-auto">
                <h2 className="pageTitle pb-4">{album.title}</h2>
            </div>
            {album.description != null && (
                <p className="pb-8">{album.description}</p>
            )}
            <PhotoGrid photos={album.photos} className="mx-auto" />
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
