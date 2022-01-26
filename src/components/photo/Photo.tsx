import * as React from 'react'
import cx from 'classnames'
import { PhotoResponse } from '../../types'
import { PHOTO_ALBUM_PHOTOS } from '../../../res/photos/albumData'
import { GatsbyImage } from 'gatsby-plugin-image'
import { isPhotoFileLandscape } from '../../utils'

type Props = {
    photoData: PhotoResponse
    onPressed?: (photoUid: string) => void
    showLarge?: boolean
    photoClassName?: string
    photoPortraitClassName?: string
    photoLandscapeClassName?: string
    containerClassName?: string
    containerPortraitClassName?: string
    containerLandscapeClassName?: string
    style?: React.CSSProperties
    imgStyle?: React.CSSProperties
}

const Photo = ({
    photoData,
    onPressed,
    showLarge,
    photoClassName,
    photoPortraitClassName,
    photoLandscapeClassName,
    containerClassName,
    containerPortraitClassName,
    containerLandscapeClassName,
    style,
    imgStyle,
}: Props) => {
    const { thumb, full, uid } = photoData

    const isLandscape = isPhotoFileLandscape(thumb)

    const localFile = showLarge ? full : thumb

    const photo = PHOTO_ALBUM_PHOTOS[uid]

    const onClickCallback = React.useCallback(() => {
        onPressed && onPressed(uid)
    }, [onPressed])

    if (photo == null) {
        return null
    }

    return (
        <div
            className={cx([
                containerClassName,
                isLandscape
                    ? containerLandscapeClassName
                    : containerPortraitClassName,
            ])}
            key={uid}
            style={style}
        >
            <GatsbyImage
                className={cx(
                    photoClassName,
                    isLandscape
                        ? photoLandscapeClassName
                        : photoPortraitClassName,
                )}
                image={localFile.childImageSharp.gatsbyImageData}
                alt={photo.alt}
                onClick={onClickCallback}
                imgStyle={imgStyle}
            />
        </div>
    )
}

export default Photo
