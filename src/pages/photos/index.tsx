import React, { useCallback, useMemo } from 'react'
import { Page } from '../../components/ui/Page'
import { graphql, Link, useStaticQuery } from 'gatsby'
import AlbumsYearGroup from '../../components/ui/AlbumsYearGroup'
import { Album } from '../../types'

type QueryResult = {
    allAlbum: {
        edges: {
            node: Album
        }[]
    }
}

export default function IndexPage() {
    const data: QueryResult = useStaticQuery(graphql`
        {
            allAlbum(sort: { date: DESC }) {
                edges {
                    node {
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
                            album {
                                title
                                date
                            }
                        }
                        date
                        description
                    }
                }
            }
        }
    `)

    const albums = useMemo(
        () => data.allAlbum.edges.map(({ node }) => node),
        [data],
    )

    const albumsByYear = useMemo(
        () =>
            albums.reduce<Record<number, Album[]>>((acc, album) => {
                const year = acc[album.year]
                if (year) {
                    return {
                        ...acc,
                        [album.year]: [...year, album],
                    }
                }

                return {
                    ...acc,
                    [album.year]: [album],
                }
            }, {}),
        [albums],
    )

    const albumYears = useMemo(
        () =>
            albums.reduce<number[]>((acc, album) => {
                if (!acc.includes(album.year)) {
                    return [...acc, album.year]
                }
                return acc
            }, []),
        [albums],
    )

    const renderYear = useCallback(
        (year: number) => {
            const albums = albumsByYear[year]
            if (!albums) {
                return null
            }

            return <AlbumsYearGroup key={year} year={year} albums={albums} />
        },
        [albumsByYear],
    )

    return (
        <Page title="Photos">
            <div className="flex justify-between items-baseline">
                <h2 className="pageTitle">Photos</h2>
                <div>
                    <Link to="/photos/tags" className="link mr-4">
                        Tags
                    </Link>
                    <Link to="/photos/all" className="link">
                        All Photos
                    </Link>
                </div>
            </div>
            {/* {featuredPhotos.length > 0 && (
                <PhotoGrid
                    photos={featuredPhotos}
                    className="mb-8"
                    onClick={onClickCallback}
                />
            )} */}
            {albumYears.map(renderYear)}
            {/* {PhotoViewerComponent} */}
        </Page>
    )
}
