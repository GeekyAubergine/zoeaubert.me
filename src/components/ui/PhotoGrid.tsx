import * as React from 'react'
import { Photo as PhotoType } from '../../../res/photos'
import Photo from './Photo'

type Props = {
    photos: PhotoType[]
    className?: string
    onClick?: (photo: PhotoType) => void,
}

export default function PhotoGrid({
    photos,
    className = '',
    onClick,
}: Props) {
    const renderPhoto = React.useCallback(
        (photo) => (
            <Photo
                photo={photo}
                key={photo.url}
                onClick={onClick}
            />
        ),
        [onClick],
    )

    return (
        <div
            className={`grid gap-x-2 gap-y-2 grid-cols-2 sm:grid-cols-3 md:grid-cols-3 ${className}`}
        >
            {photos.map(renderPhoto)}
        </div>
    )
}
