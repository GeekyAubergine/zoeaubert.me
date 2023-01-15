import { graphql, Link } from 'gatsby'
import * as React from 'react'
import SEO from '../components/Seo'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { Photo } from '../types'
import { albumToSlug } from '../utils'

type Props = {
    pageContext: {
        tag: string
    }
    data: {
        allAlbumPhoto: {
            edges: {
                node: Photo
            }[]
        }
    }
}

type Album = {
    uid: string
    title: string
    date: string
    photos: Photo[]
}

function albumUuid({ title, date }: { title: string; date: string }) {
    return `${title}-${date}`
}

function renderAlbum(album: Album) {
    return (
        <div key={album.uid} className="mb-8">
            <div className="largePhotoGrid">
                <h3 className="text-sm pb-2 sm:ml-2">
                    <Link to={albumToSlug(album)} className="link">
                        {album.date} - {album.title}
                    </Link>
                </h3>
            </div>
            <PhotoGrid photos={album.photos} />
        </div>
    )
}

export default function PhotoTagPage({ data, pageContext }: Props) {
    const { tag } = pageContext

    const { allAlbumPhoto } = data

    const photos = React.useMemo(() => {
        return allAlbumPhoto.edges.map((edge) => edge.node)
    }, [allAlbumPhoto])

    const albums: Album[] = React.useMemo(() => {
        return photos.reduce<Album[]>((acc, photo) => {
            const { album: photoAlbum } = photo

            if (!photoAlbum) {
                return acc
            }

            const uid = albumUuid(photoAlbum)
            const album = acc.find((album) => album.uid === uid)

            if (album) {
                album.photos.push(photo)
            } else {
                acc.push({
                    uid: albumUuid(photoAlbum),
                    title: photoAlbum.title,
                    date: photoAlbum.date,
                    photos: [photo],
                })
            }

            return acc
        }, [])
    }, [photos])

    return (
        <Page widthControlled={false}>
            <div className="width-control mx-auto">
                <h2 className="pageTitle pb-4">#{tag} Photos</h2>
            </div>
            {albums.map(renderAlbum)}
        </Page>
    )
}

export function Head({ data, pageContext }: Props) {
    const { tag } = pageContext

    const { allAlbumPhoto } = data

    const photos = React.useMemo(() => {
        return allAlbumPhoto.edges.map((edge) => edge.node)
    }, [allAlbumPhoto])

    return (
        <SEO
            title={`#${tag} Photos`}
            description={`Photos tagged with #${tag}`}
            image={photos[0]?.localFile.publicURL}
        />
    )
}

export const pageQuery = graphql`
    query PhotoTagPageQuery($tag: String) {
        allAlbumPhoto(
            filter: { tags: { in: [$tag] } }
            sort: { album: { date: DESC } }
        ) {
            edges {
                node {
                    id
                    description
                    alt
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
                        publicURL
                    }
                    album {
                        title
                        date
                    }
                }
            }
        }
    }
`
