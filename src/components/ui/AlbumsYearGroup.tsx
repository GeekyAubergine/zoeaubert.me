import React, { useCallback } from 'react'
import { ALBUMS_BY_YEAR, PhotoNodeData } from '../../../res/photos'
import { Album as AlbumType } from '../../types'
import Album from './Album'

type Props = {
    year: number
    albums: AlbumType[]
}

export default function AlbumsYearGroup({ year, albums }: Props) {
    const renderAlbum = useCallback(
        (album: AlbumType) => {
            return <Album key={album.uid} album={album} />
        },
        [albums],
    )

    if (!albums) {
        return null
    }

    return (
        <div key={year} className="my-2">
            <h3 className="">{year}</h3>
            <div className="grid gap-x-2 gap-y-2 grid-cols-1 sm:grid-cols-2 mb-8">
                {albums.map(renderAlbum)}
            </div>
        </div>
    )
}
