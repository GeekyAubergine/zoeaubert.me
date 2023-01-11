import * as React from 'react'
import { Album, Photo as PhotoType } from '../../types'
import Photo from './Photo'

type PhotoAndAlbum = { photo: PhotoType; album: Album }

type Props = {
    photosAndAlbums: PhotoAndAlbum[]
    className?: string
    onClick?: (photo: PhotoType) => void
}

export default function PhotoGrid({
    photosAndAlbums,
    className = '',
    onClick,
}: Props) {
    const renderPhoto = React.useCallback(
        (photoAndAlbum: PhotoAndAlbum) => (
            <div
                className="flex justify-center items-center sm:max-h-[16rem]"
                key={photoAndAlbum.photo.url}
            >
                <Photo
                    photo={photoAndAlbum.photo}
                    album={photoAndAlbum.album}
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
            {photosAndAlbums.map(renderPhoto)}
        </div>
    )
}
