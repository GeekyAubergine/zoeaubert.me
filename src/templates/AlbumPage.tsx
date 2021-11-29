import * as React from 'react'
import {graphql} from "gatsby";
import { useKeyDown } from "react-keyboard-input-hook";
import {PHOTO_ALBUM_ALBUMS} from "../../res/photos/albumData";
import Photo from "../components/photo/Photo";
import {PhotoNode} from "../types";
import BasePage from "../components/page/BasePage";
import * as styles from './albumPage.module.scss'

type QueryResponse = {
    allPhoto: {
        edges: PhotoNode[],
    }
}

type Props = {
    data: QueryResponse,
    pageContext: {
        albumUid: string,
    }
}

type PhotoData = PhotoNode & {
    landscape: boolean,
}

const AlbumPage = ({ pageContext, data }: Props) => {
    const { albumUid } = pageContext

    const [openPhotoIndex, setOpenPhotoIndex] = React.useState<number | null>(null)
    const [photoModalOpen, setPhotoModalOpen] = React.useState<boolean>(false)

    const photos = data.allPhoto.edges
    const photoOrder = React.useMemo(() => photos.map(edge => edge.node.uid), [photos])
    const album = PHOTO_ALBUM_ALBUMS[albumUid]

    const onPhotoPressedCallback = React.useCallback((photoUid: string) => {
        const index = photoOrder.findIndex((uid) => uid === photoUid)
        if (index >= 0) {
            setOpenPhotoIndex(index)
            setPhotoModalOpen(true)
        }
    }, [])

    const keyDownCallback = React.useCallback(({ keyName }) => {
        if (photoModalOpen) {
            if (keyName === 'ArrowLeft') {
                setOpenPhotoIndex(Math.max(openPhotoIndex - 1, 0))
            } else if (keyName === 'ArrowRight') {
                setOpenPhotoIndex(Math.min(openPhotoIndex + 1, photoOrder.length - 1))
            } else if (keyName === 'Escape') {
                setOpenPhotoIndex(null)
                setPhotoModalOpen(false)
            }
        }
    }, [photoModalOpen, openPhotoIndex])

    const renderPhotoCallback = React.useCallback((photoData: PhotoData) => (
            <Photo
                key={photoData.node.uid}
                photoData={photoData.node}
                onPressed={onPhotoPressedCallback}
                photoClassName={styles.thumb}
                photoPortraitClassName={styles.thumbPortrait}
                photoLandscapeClassName={styles.thumbLandscape}
                containerClassName={styles.thumbContainer}
                containerPortraitClassName={styles.thumbContainerPortrait}
                containerLandscapeClassName={styles.thumbContainerLandscape}
            />
    ), [onPhotoPressedCallback])

    useKeyDown(keyDownCallback)

    const currentPhoto = photos[openPhotoIndex]

    if (album == null) {
        return null
    }

    return (
        <BasePage title={album.description} description={album.description}>
            <div className={styles.container}>
                <div className={styles.grid}>
                    {photos.map(renderPhotoCallback)}
                </div>
            </div>
            {photoModalOpen && currentPhoto != null && (
                <div className={styles.fullScreenModal}>
                    <div className={styles.exit}>
                        X
                    </div>
                    <div className={styles.fullWrapper}>
                        <Photo
                            photoData={currentPhoto.node}
                            photoClassName={styles.full}
                            photoPortraitClassName={styles.fullPortrait}
                            photoLandscapeClassName={styles.fullLandscape}
                            containerClassName={styles.fullContainer}
                            containerPortraitClassName={styles.fullContainerPortrait}
                            containerLandscapeClassName={styles.fullContainerLandscape}
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
            )}
        </BasePage>
    )
}

export default AlbumPage

export const pageQuery = graphql`
    query AlbumPageQuery($albumUid: String = "") {
        allPhoto(filter: {albumUid: {eq: $albumUid}}) {
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