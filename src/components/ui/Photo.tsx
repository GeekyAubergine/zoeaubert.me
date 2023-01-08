import { Link, useStaticQuery } from 'gatsby'
import { graphql } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import * as React from 'react'
import {
    Photo as PhotoType,
    PhotoWithAlbum,
    photoAndAlbumToSlug,
    PhotoNodeData,
} from '../../../res/photos'

type Props = {
    photo: PhotoWithAlbum
    photoNodeData: PhotoNodeData[]
    fullSize?: boolean
    className?: string
    disableLink?: boolean
    onClick?: (photo: PhotoType) => void
}

export default function Photo({
    photo: photoProp,
    photoNodeData,
    className = '',
    onClick,
    disableLink = false,
}: Props) {
    const { photo, album } = photoProp

    // const allImages = useStaticQuery(
    //     graphql`
    //         {
    //             allFile(filter: { dir: { regex: "/images/" } }) {
    //                 edges {
    //                     node {
    //                         relativePath
    //                         childImageSharp {
    //                             gatsbyImageData
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     `,
    // )

    const cleanedPath = photo.path.replace(/^\//, '')

    const imageNode = photoNodeData.find(
        (node) => node.relativePath === cleanedPath,
    )

    if (!imageNode) {
        return null
    }

    const image = getImage(imageNode)

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
