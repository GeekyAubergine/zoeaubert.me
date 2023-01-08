import * as React from 'react'
import { ALBUMS_BY_YEAR } from '../../../res/photos'
import Album from './Album'

type Props = {
    year: number
}

function renderAlbum(uuid: string) {
    return <Album uuid={uuid} key={uuid} />
}

export default function AlbumsYearGroup({ year }: Props) {
    const albums = ALBUMS_BY_YEAR[year]

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
