import { Link, useStaticQuery } from 'gatsby'
import { graphql } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import * as React from 'react'
import { Album as AlbumType, Photo as PhotoType } from '../../types'
import { photoAndAlbumToSlug } from '../../utils'

type Props = {
    photo: PhotoType
    album: AlbumType
    fullSize?: boolean
    className?: string
    disableLink?: boolean
    onClick?: (photo: PhotoType) => void
}

export default function Photo({
    photo,
    album,
    className = '',
    onClick,
    disableLink = false,
}: Props) {
    const image = getImage(photo.localFile)

    if (!image) {
        return null
    }

    if (disableLink || !album) {
        return (
            <GatsbyImage
                key={photo.url}
                className={`m-auto ${className} ${
                    onClick != null ? 'cursor-pointer' : ''
                }`}
                image={image}
                loading="lazy"
                alt={photo.description}
            />
        )
    }

    return (
        <Link to={photoAndAlbumToSlug(album, photo)}>
            <GatsbyImage
                key={photo.url}
                className={`m-auto ${className} ${
                    onClick != null ? 'cursor-pointer' : ''
                }`}
                image={image}
                loading="lazy"
                alt={photo.description}
            />
        </Link>
    )
}
