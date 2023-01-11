import React, { useCallback } from 'react'
import { Page } from '../../components/ui/Page'
import { graphql, Link, useStaticQuery } from 'gatsby'
import AlbumsYearGroup from '../../components/ui/AlbumsYearGroup'
import { usePhotoNodeData } from '../../utils'
import { Album } from '../../types'

const MAX_FEATURED_PHOTOS = 9

type QueryResult = {
    allAlbum: {
        edges: {
            node: Album
        }[]
    }
}

export default function IndexPage() {
    // const featuredPhotos: PhotoType[] = React.useMemo(() => {
    //     return ALBUMS_BY_DATE.reduce(
    //         (acc: PhotoType[], album) =>
    //             acc.concat(album.photos.filter((photo) => photo.featured)),
    //         [],
    //     ).slice(0, MAX_FEATURED_PHOTOS)
    // }, [])

    // const { onPhotoClick, Component: PhotoViewerComponent } = usePhotoViewer({
    //     photos: featuredPhotos,
    // })

    // const onClickCallback = React.useCallback(
    //     (photo: PhotoType) => {
    //         onPhotoClick(photo)
    //     },
    //     [onPhotoClick],
    // )
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
                        }
                        date
                        description
                    }
                }
            }
        }
    `)

    const albums = data.allAlbum.edges.map(({ node }) => node)

    const albumsByYear = albums.reduce<Record<number, Album[]>>(
        (acc, album) => {
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
        },
        {},
    )

    const albumYears = albums.reduce<number[]>((acc, album) => {
        if (!acc.includes(album.year)) {
            return [...acc, album.year]
        }
        return acc
    }, [])

    const photoNodeData = usePhotoNodeData()

    const renderYear = useCallback(
        (year: number) => {
            const albums = albumsByYear[year]
            if (!albums) {
                return null
            }

            return <AlbumsYearGroup key={year} year={year} albums={albums} />
        },
        [photoNodeData],
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
