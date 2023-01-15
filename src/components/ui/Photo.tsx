import { Link, useStaticQuery } from 'gatsby'
import { graphql } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import * as React from 'react'
import { Album as AlbumType, Photo as PhotoType } from '../../types'
import { photoAndAlbumToSlug } from '../../utils'

type Props = {
    photo: PhotoType
    fullSize?: boolean
    className?: string
    disableLink?: boolean
    onClick?: (photo: PhotoType) => void
}

export default function Photo({
    photo,
    className = '',
    onClick,
    disableLink = false,
}: Props) {
    const image = getImage(photo.localFile)

    if (!image) {
        return null
    }

    if (disableLink || !photo.album) {
        return (
            <GatsbyImage
                key={photo.url}
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
        <Link to={photoAndAlbumToSlug(photo.album, photo)}>
            <GatsbyImage
                key={photo.url}
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
