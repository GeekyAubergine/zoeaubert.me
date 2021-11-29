import * as React from "react";
import cx from 'classnames'
import { PhotoResponse} from "../../types";
import {PHOTO_ALBUM_PHOTOS} from "../../../res/photos/albumData";
import {GatsbyImage} from "gatsby-plugin-image";

type Props = {
    photoData: PhotoResponse,
    onPressed?: (photoUid: string) => void,
    showLarge?: boolean
    photoClassName?: string,
    photoPortraitClassName?: string,
    photoLandscapeClassName?: string,
    containerClassName?: string,
    containerPortraitClassName?: string,
    containerLandscapeClassName?: string,
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
               }: Props) => {
    const { thumb, full, uid } = photoData

    const isLandscape = thumb.childImageSharp.gatsbyImageData.width > thumb.childImageSharp.gatsbyImageData.height

    const localFile = showLarge ? full : thumb

    const photo = PHOTO_ALBUM_PHOTOS[uid]

    const onClickCallback = React.useCallback(() => {
        onPressed && onPressed(uid)
    }, [onPressed])

    if (photo == null) {
        return null
    }

    return (
        <div className={cx([containerClassName, isLandscape ? containerLandscapeClassName : containerPortraitClassName])} key={uid}>
            <GatsbyImage
                className={cx(photoClassName, isLandscape ? photoLandscapeClassName : photoPortraitClassName)}
                image={localFile.childImageSharp.gatsbyImageData}
                alt={photo.alt}
                onClick={onClickCallback}
            />
        </div>
    )
}

export default Photo