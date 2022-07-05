import * as React from 'react'
import { PHOTO_CDN_URL, Photo as PhotoType } from '../../../res/photos'

type Props = {
    photo: PhotoType
    fullSize?: boolean
    className?: string
    onClick?: (photo: PhotoType) => void
}

export default function Photo({
    photo,
    fullSize,
    className = '',
    onClick,
}: Props) {
    const url = fullSize ? photo.url : photo.url.replace('.jpg', '-min.jpg')

    const onClickCallback = React.useCallback(() => {
        if (onClick) {
            onClick(photo)
        }
    }, [onClick, photo])

    return (
        <img
            onClick={onClickCallback}
            className={`mx-auto my-auto ${className} ${
                onClick != null ? 'cursor-pointer' : ''
            }`}
            src={`${PHOTO_CDN_URL}${url}`}
            loading="lazy"
            alt={photo.alt}
        />
    )
}
