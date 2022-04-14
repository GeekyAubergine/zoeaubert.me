import * as React from 'react'
import { Link } from 'gatsby'
import * as styles from './albumsPage.module.scss'
import BasePage from '../../components/page/BasePage'
import Photo from '../../components/photo/Photo'
import { AlbumNode, PhotoNode, PhotoResponse } from '../../types'
import { useDimension } from '../../hooks/useDimension'
import { Album as AlbumType } from '../../../res/photos/albumData'
import { isPhotoFilePortrait } from '../../utils'

type QueryResponse = {
    allPhoto: {
        edges: PhotoNode[]
    }
    allAlbum: {
        edges: AlbumNode[]
    }
}

type Props = {
    data: QueryResponse
}

type AlbumData = AlbumType & {
    coverPhotos: PhotoResponse[]
    percentageWidth: number
}

type AlbumRowData = AlbumData[]

type AlbumsYearData = {
    year: string
    rows: AlbumRowData[]
}

const DEFAULT_ALBUMS_PER_ROW = 4
const ALBUM_MIN_WIDTH = 240

const Album = ({ album }: { album: AlbumData }) => {
    const computedStyles = React.useMemo(
        () => ({
            maxWidth: `${(album.percentageWidth * 100 - 0.25).toFixed(6)}%`,
        }),
        [album.percentageWidth],
    )

    const coverPhoto1 = album.coverPhotos[0]
    const coverPhoto2 = album.coverPhotos[1]

    const coverPhoto1Portrait =
        coverPhoto1 != null && isPhotoFilePortrait(coverPhoto1.thumb)

    const coverPhoto2Portrait =
        coverPhoto2 != null && isPhotoFilePortrait(coverPhoto2.thumb)

    const renderBothCoverPhotos = coverPhoto1Portrait && coverPhoto2Portrait

    return (
        <Link
            className={styles.album}
            style={computedStyles}
            to={`${album.slug}`}
        >
            <div className={styles.albumNameAndCover}>
                <div className={styles.albumNameAndCover}>
                    <div className={styles.coverContainer}>
                        {coverPhoto1 && (
                            <Photo
                                key={coverPhoto1.uid}
                                photoData={coverPhoto1}
                                photoClassName={styles.thumb}
                                photoPortraitClassName={styles.thumbPortrait}
                                photoLandscapeClassName={styles.thumbLandscape}
                                containerClassName={styles.thumbContainer}
                                containerPortraitClassName={
                                    styles.thumbContainerPortrait
                                }
                                containerLandscapeClassName={
                                    styles.thumbContainerLandscape
                                }
                            />
                        )}
                        {coverPhoto2 && renderBothCoverPhotos && (
                            <Photo
                                key={coverPhoto2.uid}
                                photoData={coverPhoto2}
                                photoClassName={styles.thumb}
                                photoPortraitClassName={styles.thumbPortrait}
                                photoLandscapeClassName={styles.thumbLandscape}
                                containerClassName={styles.thumbContainer}
                                containerPortraitClassName={
                                    styles.thumbContainerPortrait
                                }
                                containerLandscapeClassName={
                                    styles.thumbContainerLandscape
                                }
                            />
                        )}
                    </div>
                </div>
                <div className={styles.albumName}>{album.name}</div>
            </div>
        </Link>
    )
}

const AlbumsRow = ({ rowData }: { rowData: AlbumRowData }) => {
    const renderAlbumCallback = React.useCallback((album: AlbumData) => {
        return <Album key={album.uid} album={album} />
    }, [])

    return (
        <div className={styles.albumRow}>
            {rowData.map(renderAlbumCallback)}
        </div>
    )
}

const AlbumsPage = ({ data }: Props) => {
    const { allAlbum, allPhoto } = data

    const wrapperRef = React.useRef(null)
    const dimension = useDimension(wrapperRef)

    const albumsPerRow = React.useMemo(
        () =>
            dimension.width === 0
                ? DEFAULT_ALBUMS_PER_ROW
                : Math.floor(dimension.width / ALBUM_MIN_WIDTH),
        [dimension.width],
    )

    const renderAlbumsRowCallback = React.useCallback(
        (rowData: AlbumRowData) => {
            return (
                <AlbumsRow
                    key={rowData.map((d) => d.uid).join('|')}
                    rowData={rowData}
                />
            )
        },
        [],
    )

    const processedData: AlbumRowData[] = React.useMemo(() => {
        const albumUids = allAlbum.edges.map((e) => e.node.uid)

        const rows: AlbumRowData[] = albumUids.reduce(
            (
                rowAcc: AlbumRowData[],
                albumUid: string,
            ): AlbumRowData[] => {
                const album: AlbumNode | null = allAlbum.edges.find(
                    (e) => e.node.uid === albumUid,
                )

                if (album == null) {
                    return null
                }

                const coverPhotos = album.node.cover_photo_uids
                    .map((coverUid: string) =>
                        allPhoto.edges.find(
                            (e) => e.node.uid === coverUid,
                        ),
                    )
                    .filter((e) => e != null)
                    .map((e) => e.node)

                if (coverPhotos.length < 1) {
                    return null
                }

                const albumData: AlbumData = {
                    ...album.node,
                    coverPhotos: coverPhotos,
                    percentageWidth: 1 / albumsPerRow,
                }

                return rowAcc[rowAcc.length - 1] &&
                    rowAcc[rowAcc.length - 1].length < albumsPerRow
                    ? [
                            ...rowAcc.slice(0, rowAcc.length - 1),
                            [...rowAcc[rowAcc.length - 1], albumData],
                        ]
                    : [...rowAcc, [albumData]]
            },
            [],
        )

        return rows
    }, [allAlbum, albumsPerRow])

    return (
        <BasePage title="Albums" description="Albums">
            <div ref={wrapperRef} className={styles.wrapper}>
                <h2 className={styles.pageTitle}>Photos</h2>
                <div className={styles.container}>
                    {processedData.map(renderAlbumsRowCallback)}
                </div>
            </div>
        </BasePage>
    )
}

export default AlbumsPage
