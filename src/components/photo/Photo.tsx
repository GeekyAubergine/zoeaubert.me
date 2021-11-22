import * as React from "react";
import cx from 'classnames'
import { PhotoResponse} from "../../types";
import {PHOTO_ALBUM_PHOTOS} from "../../../res/photos/albumData";
import {GatsbyImage} from "gatsby-plugin-image";

type Props = {
    photoData: PhotoResponse,
    onPressed?: (photoUid: string) => void,
    showLarge?: boolean
    className?: string,
}

const Photo = ({ photoData, onPressed, showLarge, className }: Props) => {
    const { smallPhoto, largePhoto, uid } = photoData

    const localFile = showLarge ? largePhoto : smallPhoto

    const photo = PHOTO_ALBUM_PHOTOS[uid]

    const onClickCallback = React.useCallback(() => {
        onPressed && onPressed(uid)
    }, [onPressed])

    if (photo == null) {
        return null
    }

    console.log({ className })

    return (
        <GatsbyImage
            className={className}
            image={localFile.childImageSharp.gatsbyImageData}
            alt={photo.alt}
            onClick={onClickCallback}
        />
    )
}

export default Photo