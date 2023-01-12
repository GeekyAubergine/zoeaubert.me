import { graphql } from 'gatsby'
import * as React from 'react'
import { Page } from '../components/ui/Page'
import PhotoGrid from '../components/ui/PhotoGrid'
import { Photo } from '../types'

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

export default function PhotoTagPage({ data, pageContext }: Props) {
    const { tag } = pageContext

    const { allAlbumPhoto } = data

    const photos = React.useMemo(() => {
        return allAlbumPhoto.edges.map((edge) => edge.node)
    }, [allAlbumPhoto])

    return (
        <Page title="Photos">
            <h2 className="pageTitle pb-4">#{tag}</h2>
            <PhotoGrid
                photos={photos}
                className="mb-8"
            />
        </Page>
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
                    album {
                        title
                        date
                    }
                }
            }
        }
    }
`
