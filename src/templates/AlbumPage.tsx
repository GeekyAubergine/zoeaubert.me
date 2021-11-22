import * as React from 'react'
import {graphql} from "gatsby";
import { useKeyDown } from "react-keyboard-input-hook";
import cx from 'classnames'
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

    const [openPhotoIndex, setOpenPhotoIndex] = React.useState<number | null>(1)
    const [photoModalOpen, setPhotoModalOpen] = React.useState<boolean>(true)

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

    useKeyDown(keyDownCallback)

    const renderPhotoCallback = React.useCallback((photoData: PhotoData) => (
            <Photo
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
            {photoModalOpen && (
                <div className={styles.fullScreenModal}>
                    {currentPhoto != null && (
                        <Photo
                            photoData={currentPhoto.node}
                            photoClassName={styles.fullContainer}
                            photoPortraitClassName={styles.fullContainerPortrait}
                            photoLandscapeClassName={styles.fullContainerLandscape}
                            containerClassName={styles.full}
                            containerPortraitClassName={styles.fullPortrait}
                            containerLandscapeClassName={styles.fullLandscape}
                            showLarge
                        />
                    )}
                </div>
            )}
        </BasePage>
    )
}

export default AlbumPage

export const pageQuery = graphql`
query MyQuery {
  allPhoto {
    edges {
      node {
        uid
        alt
        thumb: localFile {
          childImageSharp {
            gatsbyImageData(width: 200)
          }
        }
        full: localFile {
          childImageSharp {
                gatsbyImageData(width: 2000)
          }
        }
      }
    }
  }
}
`