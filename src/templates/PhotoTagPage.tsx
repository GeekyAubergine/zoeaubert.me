import { graphql } from 'gatsby'
import * as React from 'react'
import SEO from '../components/Seo'
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
        <Page widthControlled={false}>
            <div className="width-control mx-auto">
                <h2 className="pageTitle pb-4">#{tag}</h2>
            </div>
            <PhotoGrid photos={photos} className="mx-auto" />
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
