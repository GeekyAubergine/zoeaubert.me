import * as React from 'react'
import { Album, Photo as PhotoType } from '../../types'
import Photo from './Photo'

type Props = {
    photos: PhotoType[]
    className?: string
    onClick?: (photo: PhotoType) => void
}

export default function PhotoGrid({ photos, className = '', onClick }: Props) {
    const sortedPhotos = React.useMemo(() => {
        return photos.sort((a, b) => a.photoIndex - b.photoIndex)
    }, [photos])

    const renderPhoto = React.useCallback(
        (photo: PhotoType) => (
            <div
                className="flex justify-center items-center sm:max-h-[16rem]"
                key={photo.url}
            >
                <Photo
                    photo={photo}
                    onClick={onClick}
                    className="sm:max-h-[16rem]"
                />
            </div>
        ),
        [onClick],
    )

    return (
        <div
            className={`grid gap-x-2 gap-y-2 grid-cols-1 sm:grid-cols-2 ${className}`}
        >
            {sortedPhotos.map(renderPhoto)}
        </div>
    )
}
