import { Link, useStaticQuery } from 'gatsby'
import { graphql } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import * as React from 'react'
import {
    Photo as PhotoType,
    PhotoAndAlbum,
    photoAndAlbumToSlug,
} from '../../../res/photos'

type Props = {
    photoAndAlbum: PhotoAndAlbum
    className?: string
    disableLink?: boolean
    onClick?: (photo: PhotoType) => void
}

export default function ThumbnailPhoto({
    photoAndAlbum,
    className = '',
    onClick,
    disableLink = false,
}: Props) {
    const { photo, album } = photoAndAlbum

    const allImages = useStaticQuery(
        graphql`
            {
                allFile(filter: { dir: { regex: "/images/" } }) {
                    edges {
                        node {
                            relativePath
                            childImageSharp {
                                gatsbyImageData(
                                    width: 300
                                )
                            }
                        }
                    }
                }
            }
        `,
    )

    const cleanedPath = photo.path.replace(/^\//, '')

    const imageNode = allImages.allFile.edges.find(
        (edge) => edge.node.relativePath === cleanedPath,
    )

    if (!imageNode) {
        return null
    }

    const image = getImage(imageNode.node)

    if (!image) {
        return null
    }


    if (disableLink || !album) {
        return (
            <GatsbyImage
                key={photo.path}
                className={`m-auto ${className} ${
                    onClick != null ? 'cursor-pointer' : ''
                }`}
                image={image}
                loading="lazy"
                alt={photo.alt}
            />
        )
    }

    return (
        <Link to={photoAndAlbumToSlug(album, photo)}>
            <GatsbyImage
                key={photo.path}
                className={`m-auto ${className} ${
                    onClick != null ? 'cursor-pointer' : ''
                }`}
                image={image}
                loading="lazy"
                alt={photo.alt}
            />
        </Link>
    )
}
