import { Link, useStaticQuery } from 'gatsby'
import { graphql } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import * as React from 'react'
import {
    Photo as PhotoType,
    PhotoAndAlbum,
    photoAndAlbumToSlug,
} from '../../../res/photos'
import FullsizePhoto from './FullsizePhoto'
import ThumbnailPhoto from './ThumbnailPhoto'

type Props = {
    photoAndAlbum: PhotoAndAlbum
    fullSize?: boolean
    className?: string
    disableLink?: boolean
    onClick?: (photo: PhotoType) => void
}

export default function Photo({
    photoAndAlbum,
    className = '',
    onClick,
    disableLink = false,
    fullSize = false,
}: Props) {
    if (fullSize) {
        return (
            <FullsizePhoto
                photoAndAlbum={photoAndAlbum}
                className={className}
                onClick={onClick}
                disableLink={disableLink}
            />
        )
    }

    return (
        <ThumbnailPhoto
            photoAndAlbum={photoAndAlbum}
            className={className}
            onClick={onClick}
            disableLink={disableLink}
        />
    )
}
