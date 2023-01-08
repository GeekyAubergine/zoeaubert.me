import * as React from 'react'
import {
    Photo as PhotoType,
    PhotoNodeData,
    PhotoWithAlbum,
} from '../../../res/photos'
import Photo from './Photo'

type Props = {
    photos: PhotoWithAlbum[]
    photoNodeData: PhotoNodeData[]
    className?: string
    onClick?: (photo: PhotoType) => void
}

export default function PhotoGrid({
    photos,
    photoNodeData,
    className = '',
    onClick,
}: Props) {
    const renderPhoto = React.useCallback(
        (photo: PhotoWithAlbum) => (
            <div
                className="flex justify-center items-center sm:max-h-[16rem]"
                key={photo.photo.path}
            >
                <Photo
                    photo={photo}
                    photoNodeData={photoNodeData}
                    onClick={onClick}
                    className="sm:max-h-[16rem]"
                />
            </div>
        ),
        [onClick, photoNodeData],
    )

    return (
        <div
            className={`grid gap-x-2 gap-y-2 grid-cols-1 sm:grid-cols-2 ${className}`}
        >
            {photos.map(renderPhoto)}
        </div>
    )
}
