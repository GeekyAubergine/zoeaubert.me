import { navigate } from 'gatsby'
import { graphql, Link } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import React from 'react'
import { useCallback } from 'react'
import {
    ALBUMS_BY_UUID,
    albumToSlug,
    photoAndAlbumToSlug,
} from '../../res/photos'
import NavBar from '../components/ui/NavBar'
import { Page } from '../components/ui/Page'
import ThemeToggle from '../components/ui/ThemeToggle'

type Props = {
    data: any
    pageContext: {
        albumUuid: string
        photoPath: string
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
    const { albumUuid, photoPath } = pageContext
    const { file } = data
    const { publicURL } = file
    const image = getImage(file)

    const album = ALBUMS_BY_UUID[albumUuid]

    const photoIndex = album.photos.findIndex(
        (photo) => photo.path === photoPath,
    )
    const photo = album.photos[photoIndex]

    if (!photo || photoIndex === -1) {
        return null
    }

    const previousPhoto = album.photos[photoIndex - 1] ?? null
    const nextPhoto = album.photos[photoIndex + 1] ?? null

    const onKeyUp = React.useCallback(
        (event) => {
            if (event.key === 'ArrowLeft' && previousPhoto) {
                navigate(photoAndAlbumToSlug(album, previousPhoto))
            }
            if (event.key === 'ArrowRight' && nextPhoto) {
                navigate(photoAndAlbumToSlug(album, nextPhoto))
            }
        },
        [previousPhoto, nextPhoto],
    )

    React.useEffect(() => {
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

    if (!photo) {
        return null
    }

    const totalPhotosDigits = album.photos.length.toString().length

    return (
        <Page
            title={`${album.title} | Photos`}
            description={photo.alt}
            image={publicURL}
            hideNavBar
            hideFooter
            widthControlled={false}
            mainClassName="h-screen max-h-screen justify-between sm:mx-4"
            preventIndexing
        >
            <div className="flex justify-between items-center mb-4 sm:hidden">
                <div className="flex items-center">
                    <Link
                        className="text-2xl h-full text-center sm:text-left"
                        to="/"
                    >
                        <h1>Zoe Aubert</h1>
                    </Link>
                    <ThemeToggle />
                </div>
                <Link className="navbarLink" to="/photos">
                    Photos
                </Link>
            </div>
            <div className="hidden sm:flex width-control mx-auto">
                <NavBar />
            </div>
            <GatsbyImage
                key={photo.path}
                image={image}
                loading="lazy"
                alt={photo.alt}
            />

            <div className="flex flex-col justify-between items-center sm:mb-8 sm:width-control sm:mx-auto">
                <div className="flex flex-col justify-between items-center">
                    <p className="w-full text-center mt-4 mb-2">{photo.alt}</p>
                    <div className="flex w-full flex-wrap justify-center">
                        {photo.tags.map(renderTag)}
                    </div>
                </div>
                <div className="flex w-full justify-between my-2">
                    {previousPhoto != null ? (
                        <Link
                            to={photoAndAlbumToSlug(album, previousPhoto)}
                            className="flex flex-1 text-center link no-underline"
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
                            className="flex flex-1 justify-end text-center link no-underline"
                        >
                            →
                        </Link>
                    ) : (
                        <div className="flex flex-1" />
                    )}
                </div>
                <div className="flex w-full justify-center items-baseline">
                    {photo != null && (
                        <>
                            <Link
                                to={albumToSlug(album)}
                                className="text-center link mt-2"
                            >
                                Rest of Album
                            </Link>
                            <p className="mx-2"> - </p>
                        </>
                    )}
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
    query ($fileName: String!) {
        file(name: { eq: $fileName }) {
            childImageSharp {
                gatsbyImageData
            }
            publicURL
        }
    }
`
