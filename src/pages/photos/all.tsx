import { graphql, Link, useStaticQuery } from 'gatsby'
import * as React from 'react'
import { Page } from '../../components/ui/Page'
import PhotoGrid from '../../components/ui/PhotoGrid'
import { Album } from '../../types'
import { albumToSlug } from '../../utils'

type Result = {
    allAlbum: {
        edges: {
            node: Album
        }[]
    }
}

function renderAlbum(album: Album) {
    return (
        <div key={album.uid} className="mb-8">
            <h3 className="text-sm pb-2">
                <Link to={albumToSlug(album)} className="link">
                    {album.date} - {album.title}
                </Link>
            </h3>
            <PhotoGrid photos={album.photos} />
        </div>
    )
}

export default function AllPhotos() {
    const result: Result = useStaticQuery(graphql`
        {
            allAlbum(sort: { date: DESC }) {
                edges {
                    node {
                        photos {
                            id
                            description
                            featured
                            tags
                            url
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
                        date(formatString: "YYYY-MM-DD")
                        title
                    }
                }
            }
        }
    `)

    const albums = React.useMemo(() => {
        return result.allAlbum.edges.map((edge) => edge.node)
    }, [result])

    return (
        <Page title="Photos">
            <h2 className="pageTitle mb-4">All Photos</h2>
            {albums.map(renderAlbum)}
        </Page>
    )
}
