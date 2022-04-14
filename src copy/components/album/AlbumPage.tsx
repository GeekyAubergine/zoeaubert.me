import * as React from 'react'
import { graphql } from 'gatsby'
import { useKeyDown } from 'react-keyboard-input-hook'
import { PHOTO_ALBUM_ALBUMS } from '../../../res/photos/albumData'
import Photo from '../photo/Photo'
import { PhotoNode, PhotoResponse } from '../../types'
import BasePage from '../page/BasePage'
import * as styles from './albumPage.module.scss'
import { useDimension } from '../../hooks/useDimension'
import { useWindowSize } from '../../hooks/useWindowSize'

type QueryResponse = {
    allPhoto: {
        edges: PhotoNode[]
    }
}

type Props = {
    data: QueryResponse
    pageContext: {
        albumUid: string
    }
}

type PhotoData = PhotoResponse & {
    landscape: boolean
}

type ColumnData = {
    photos: PhotoData[]
    height: number
    width: number
}

type RowData = {
    photos: PhotoData[]
    maxPhotoWidthUnits: number
    photosWidthUnits: number // Portrait 1, Lanscape 1.5
}

const PHOTO_WIDTH_UNITS = 1
const LANDSCAPE_WIDTH_UNITS = 2 // Should be 2.25 but 2 looks better
const PHOTO_ASPECT_RATION = 1.5
const DEFAULT_COLUMNS = 5
const COLUMN_MIN_WIDTH = 200

const firstShortestColumnIndex = (columns: ColumnData[]): number => {
    let firstShortestColumnIndex = columns.length - 1
    let shortestHeight =
        columns[firstShortestColumnIndex] != null
            ? columns[firstShortestColumnIndex].height
            : Number.MAX_SAFE_INTEGER

    for (let i = columns.length - 2; i >= 0; i--) {
        const column = columns[i]

        if (column != null && column.height <= shortestHeight) {
            firstShortestColumnIndex = i
        }
    }

    return firstShortestColumnIndex
}

const findPhotoRowIndexToInsertTo = (
    rows: RowData[],
    photo: PhotoData,
): number => {
    const photoWidthUnits = photo.landscape
        ? LANDSCAPE_WIDTH_UNITS
        : PHOTO_WIDTH_UNITS

    for (let i = 0; i < rows.length; i++) {
        const row = rows[i]

        if (row.photosWidthUnits + photoWidthUnits <= row.maxPhotoWidthUnits) {
            return i
        }
    }

    return rows.length
}

const GridPhoto = ({
    photoData,
    photoWidth,
    onPhotoPressedCallback,
}: {
    photoData: PhotoData
    photoWidth: number
    onPhotoPressedCallback: (photoUid: string) => void
}) => {
    const landscape =
        photoData.thumb.childImageSharp.gatsbyImageData.width >
        photoData.thumb.childImageSharp.gatsbyImageData.height

    const computedStyles = React.useMemo(() => {
        const width = landscape
            ? photoWidth * LANDSCAPE_WIDTH_UNITS
            : photoWidth
        const height = landscape
            ? width / PHOTO_ASPECT_RATION
            : width * PHOTO_ASPECT_RATION

        return {
            maxWidth: width,
            height,
            maxHeight: height,
        }
    }, [photoWidth, landscape])

    const onPressedCallback = React.useCallback(
        () => onPhotoPressedCallback(photoData.uid),
        [onPhotoPressedCallback, photoData],
    )

    return (
        <div
            className={styles.thumbContainer}
            style={computedStyles}
            onClick={onPressedCallback}
        >
            <Photo
                key={photoData.uid}
                photoData={photoData}
                containerClassName={styles.thumbContainer}
                style={computedStyles}
                imgStyle={computedStyles}
            />
            <div className={styles.thumbAltContainer}>
                <p className={styles.thumbAltText}>{photoData.alt}</p>
            </div>
        </div>
    )
}

const GridRow = ({
    rowData,
    photoWidth,
    onPhotoPressedCallback,
}: {
    rowData: RowData
    photoWidth: number
    onPhotoPressedCallback: (photoUid: string) => void
}) => {
    const renderPhotoCallback = React.useCallback(
        (photoData: PhotoData) => (
            <GridPhoto
                key={photoData.uid}
                photoData={photoData}
                photoWidth={photoWidth}
                onPhotoPressedCallback={onPhotoPressedCallback}
            />
        ),
        [photoWidth, onPhotoPressedCallback],
    )

    return (
        <div className={styles.row}>
            {rowData.photos.map(renderPhotoCallback)}
        </div>
    )
}

const AlbumPage = ({ pageContext, data }: Props) => {
    const { albumUid } = pageContext

    const wrapperRef = React.useRef(null)
    const dimension = useDimension(wrapperRef)
    const columns = React.useMemo(
        () =>
            dimension.width === 0
                ? DEFAULT_COLUMNS
                : Math.max(2, Math.floor(dimension.width / COLUMN_MIN_WIDTH)),
        [dimension.width],
    )

    const { width: windowWidth } = useWindowSize()
    const photoWidth = React.useMemo(
        () => Math.floor(dimension.width / columns),
        [columns, dimension.width, windowWidth],
    )

    const [openPhotoIndex, setOpenPhotoIndex] = React.useState<number | null>(
        null,
    )
    const [photoModalOpen, setPhotoModalOpen] = React.useState<boolean>(false)

    const photos = data.allPhoto.edges
    const photoOrder = React.useMemo(
        () => photos.map((edge) => edge.node.uid),
        [photos],
    )
    const album = PHOTO_ALBUM_ALBUMS[albumUid]

    const onPhotoPressedCallback = React.useCallback((photoUid: string) => {
        const index = photoOrder.findIndex((uid) => uid === photoUid)
        if (index >= 0) {
            setOpenPhotoIndex(index)
            setPhotoModalOpen(true)
        }
    }, [])

    const closeModalCallback = React.useCallback(() => {
        setOpenPhotoIndex(null)
        setPhotoModalOpen(false)
    }, [])

    const keyDownCallback = React.useCallback(
        ({ keyName }) => {
            if (photoModalOpen) {
                if (keyName === 'ArrowLeft') {
                    setOpenPhotoIndex(Math.max(openPhotoIndex - 1, 0))
                } else if (keyName === 'ArrowRight') {
                    setOpenPhotoIndex(
                        Math.min(openPhotoIndex + 1, photoOrder.length - 1),
                    )
                } else if (keyName === 'Escape') {
                    closeModalCallback()
                }
            }
        },
        [photoModalOpen, openPhotoIndex, closeModalCallback],
    )

    useKeyDown(keyDownCallback)

    const rowsData: RowData[] = React.useMemo(() => {
        const rows: RowData[] = []

        photos.forEach((node: PhotoNode, i: number) => {
            const landscape =
                node.node.thumb.childImageSharp.gatsbyImageData.width >
                node.node.thumb.childImageSharp.gatsbyImageData.height

            const photoWidthUnits = landscape
                ? LANDSCAPE_WIDTH_UNITS
                : PHOTO_WIDTH_UNITS

            const photoData = {
                ...node.node,
                landscape,
            }

            const insertIndex = findPhotoRowIndexToInsertTo(rows, photoData)

            if (rows.length > insertIndex) {
                rows[insertIndex] = {
                    ...rows[insertIndex],
                    photosWidthUnits:
                        rows[insertIndex].photosWidthUnits + photoWidthUnits,
                    photos: [...rows[insertIndex].photos, photoData],
                }
            } else {
                rows.push({
                    photos: [photoData],
                    photosWidthUnits: photoWidthUnits,
                    maxPhotoWidthUnits: columns,
                })
            }
        })

        return rows
    }, [columns])

    const currentPhoto = photos[openPhotoIndex]

    const renderRowCallback = React.useCallback(
        (rowData: RowData) => (
            <GridRow
                key={rowData.photos.map((p) => p.uid).join('|')}
                rowData={rowData}
                photoWidth={photoWidth}
                onPhotoPressedCallback={onPhotoPressedCallback}
            />
        ),
        [photoWidth, onPhotoPressedCallback],
    )

    if (album == null) {
        return null
    }

    return (
        <BasePage title={album.description} description={album.description}>
            <div className={styles.container} ref={wrapperRef}>
                <div className={styles.grid}>
                    {rowsData.map(renderRowCallback)}
                </div>
            </div>
            {photoModalOpen && currentPhoto != null && (
                <div className={styles.fullScreenModal}
                onTouchStart={console.log}>
                    <div className={styles.fullScreenModalContents}>
                        <div
                            className={styles.exit}
                            onClick={closeModalCallback}
                        >
                            X
                        </div>
                        <div className={styles.fullWrapper}>
                            <Photo
                                photoData={currentPhoto.node}
                                photoClassName={styles.full}
                                photoPortraitClassName={styles.fullPortrait}
                                photoLandscapeClassName={styles.fullLandscape}
                                containerClassName={styles.fullContainer}
                                containerPortraitClassName={
                                    styles.fullContainerPortrait
                                }
                                containerLandscapeClassName={
                                    styles.fullContainerLandscape
                                }
                                showLarge
                            />
                        </div>
                        <div className={styles.details}>
                            <div className={styles.description}>
                                <p>{currentPhoto.node.alt}</p>
                            </div>
                            <p className={styles.counter}>
                                {`${openPhotoIndex + 1} / ${photos.length}`}
                            </p>
                        </div>
                    </div>
                </div>
            )}
        </BasePage>
    )
}

export default AlbumPage

export const pageQuery = graphql`
    query AlbumPageQuery($albumUid: String = "") {
        allPhoto(filter: { albumUid: { eq: $albumUid } }) {
            edges {
                node {
                    uid
                    alt
                    thumb: localFile {
                        childImageSharp {
                            gatsbyImageData(height: 200, layout: CONSTRAINED)
                        }
                    }
                    full: localFile {
                        childImageSharp {
                            gatsbyImageData(height: 2000, layout: CONSTRAINED)
                        }
                    }
                }
            }
        }
    }
`
