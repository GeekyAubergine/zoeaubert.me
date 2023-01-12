import { navigate } from 'gatsby'
import { graphql, Link } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import React, { useCallback, useEffect } from 'react'
import { Page } from '../components/ui/Page'
import ThemeToggle from '../components/ui/ThemeToggle'
import { Album, Photo } from '../types'
import { albumToSlug, photoAndAlbumToSlug } from '../utils'

type Props = {
    data: {
        albumPhoto: Photo | null
        album: Album | null
    }
    pageContext: {
        albumId: string
        photoId: string
    }
}

function renderTag(tag: string) {
    return (
        <Link
            to={`/photos/tags/${tag}`}
            className="m-1 link no-underline"
            key={tag}
        >
            #{tag}
        </Link>
    )
}

export default function PhotoPage({ data, pageContext }: Props) {
    const { albumPhoto, album } = data
    const { photoId } = pageContext

    if (!album || !albumPhoto) {
        return null
    }

    const { localFile, description, url, tags } = albumPhoto

    const { publicURL } = localFile
    const image = getImage(localFile)

    const photoIndex = album.photos.findIndex((photo) => photo.id === photoId)

    const previousPhoto = album.photos[photoIndex - 1] ?? null
    const nextPhoto = album.photos[photoIndex + 1] ?? null

    const goBack = useCallback(() => {
        navigate(-1)
    }, [])

    console.log( { localFile })

    const onKeyUp = useCallback(
        (event) => {
            if (event.key === 'Escape') {
                goBack()
            }
            if (event.key === 'ArrowLeft' && previousPhoto) {
                navigate(photoAndAlbumToSlug(album, previousPhoto), {
                    replace: true,
                })
            }
            if (event.key === 'ArrowRight' && nextPhoto) {
                navigate(photoAndAlbumToSlug(album, nextPhoto), {
                    replace: true,
                })
            }
        },
        [previousPhoto, nextPhoto],
    )

    useEffect(() => {
        if (typeof window !== 'undefined' && window.document) {
            window.addEventListener('keyup', onKeyUp)
        }
        return () => {
            if (typeof window !== 'undefined' && window.document) {
                window.removeEventListener('keyup', onKeyUp)
            }
        }
    }, [])

    if (!image) {
        return null
    }

    const totalPhotosDigits = album.photos.length.toString().length

    return (
        <Page
            title={`${album.title} | Photos`}
            description={description}
            image={publicURL}
            hideNavBar
            hideFooter
            widthControlled={false}
            mainClassName="h-screen max-h-screen justify-between sm:mx-4"
            preventIndexing
        >
            <div className="flex justify-between items-center mb-4 sm:width-control sm:mx-auto">
                <div className="flex items-center">
                    <Link
                        className="text-2xl h-full text-center sm:text-left"
                        to="/"
                    >
                        <h1>Zoe Aubert</h1>
                    </Link>
                    <ThemeToggle />
                </div>
                <p className="navbarLink" onClick={goBack}>
                    Back
                </p>
            </div>
            {/* <div className="hidden sm:flex width-control mx-auto">
                <NavBar />
            </div> */}
            <GatsbyImage
                key={url}
                image={image}
                loading="lazy"
                alt={description}
            />

            <div className="flex flex-col justify-between items-center sm:mb-8 sm:width-control sm:mx-auto">
                <div className="flex flex-col justify-between items-center">
                    <p className="w-full text-center mt-4 mb-2">
                        {description}
                    </p>
                    <div className="flex w-full flex-wrap justify-center">
                        {tags.map(renderTag)}
                    </div>
                </div>
                <div className="flex w-full justify-between items-center">
                    {previousPhoto != null ? (
                        <Link
                            to={photoAndAlbumToSlug(album, previousPhoto)}
                            className="flex flex-1 text-center link no-underline py-2"
                            replace
                        >
                            ←
                        </Link>
                    ) : (
                        <div className="flex flex-1" />
                    )}
                    <p className="flex flex-1 justify-center">
                        {(photoIndex + 1)
                            .toString()
                            .padStart(totalPhotosDigits, '0')}{' '}
                        / {album.photos.length}
                    </p>
                    {nextPhoto != null ? (
                        <Link
                            to={photoAndAlbumToSlug(album, nextPhoto)}
                            className="flex flex-1 justify-end text-center link no-underline py-2"
                            replace
                        >
                            →
                        </Link>
                    ) : (
                        <div className="flex flex-1" />
                    )}
                </div>
                <div className="flex w-full justify-center items-baseline">
                    <Link
                        to={albumToSlug(album)}
                        className="text-center link mt-2"
                    >
                        Rest of Album
                    </Link>
                    <p className="mx-2"> - </p>
                    <a
                        className="link"
                        href={publicURL}
                        target="_blank"
                        rel="noopener"
                    >
                        Original
                    </a>
                </div>
            </div>
        </Page>
    )
}

export const pageQuery = graphql`
    query ($photoId: String!, $albumId: String!) {
        albumPhoto(id: { eq: $photoId }) {
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
        album(id: { eq: $albumId }) {
            id
            year
            uid
            title
            date
            description
            photos {
                id
                description
                featured
                tags
                url
            }
        }
    }
`
