import * as React from 'react'
import { Photo as PhotoType } from '../../types'
import Photo from './Photo'

const LANDSCAPE_IMAGE_WIDTH = 2
const PORTRAIT_IMAGE_WIDTH = 1

type Props = {
    photos: PhotoType[]
    className?: string
    onClick?: (photo: PhotoType) => void
}

function columWidthForPhoto(photo: PhotoType) {
    return photo.localFile.childImageSharp.original.width >
        photo.localFile.childImageSharp.original.height
        ? LANDSCAPE_IMAGE_WIDTH
        : PORTRAIT_IMAGE_WIDTH
}

function buildGrid(photos: PhotoType[], columns: number): PhotoType[] {
    const rows: { photos: PhotoType[]; columsUsed: number }[] = []

    for (const photo of photos) {
        const lastRow = rows[rows.length - 1]

        if (lastRow == null) {
            rows.push({
                photos: [photo],
                columsUsed: columWidthForPhoto(photo),
            })
            continue
        }

        const width = columWidthForPhoto(photo)

        let added = false

        for (const row of rows) {
            if (!added && row.columsUsed + width <= columns) {
                row.photos.push(photo)
                row.columsUsed += width
                added = true
            }
        }

        if (!added) {
            rows.push({
                photos: [photo],
                columsUsed: width,
            })
        }
    }

    return rows.reduce<PhotoType[]>((acc, row) => acc.concat(row.photos), [])
}

export default function PhotoGrid({
    photos,
    className = '',
    onClick,
}: Props) {
    const [columns, setColumns] = React.useState<number | null>(null)

    const sortedPhotos = React.useMemo(() => {
        console.log({ columns })

        const sorted = photos.sort((a, b) => a.photoIndex - b.photoIndex)

        if (columns == null) {
            return sorted
        }

        return buildGrid(sorted, columns)
    }, [photos, columns])

    const onResize = React.useCallback(() => {
        if (typeof window !== 'undefined' && window.document) {
            console.log('resize')
            const grid = window.document.querySelector('.photo-grid')

            if (!grid) {
                return
            }

            const computedStyle = window.getComputedStyle(grid)

            const columns = computedStyle
                .getPropertyValue('grid-template-columns')
                .split(' ').length

            setColumns(columns)
        }
    }, [])

    React.useEffect(() => {
        if (typeof window !== 'undefined' && window.document) {
            window.addEventListener('resize', () => {
                onResize()
            })
        }
    }, [onResize])

    React.useEffect(() => {
        onResize()
        setTimeout(onResize, 1000)
    }, [])

    const renderPhoto = React.useCallback(
        (photo: PhotoType) => {
            const span =
                photo.localFile.childImageSharp.original.width >
                photo.localFile.childImageSharp.original.height
                    ? `col-span-${LANDSCAPE_IMAGE_WIDTH}`
                    : `col-span-${PORTRAIT_IMAGE_WIDTH}`

            return (
                <div
                    className={`flex justify-center items-center ${span}`}
                    key={photo.url}
                >
                    <Photo photo={photo} onClick={onClick} />
                </div>
            )
        },
        [onClick],
    )

    return (
        <div className="flex w-full justify-center">
            <div
                className={`photo-grid max-w-[80rem] grid gap-x-2 gap-y-2 mx-auto px-0 grid-cols-2 md:grid-cols-4 lg:grid-cols-6 xl:grid-cols-48 sm:px-2 ${className}`}
            >
                {sortedPhotos.map(renderPhoto)}
            </div>
        </div>
    )
}
