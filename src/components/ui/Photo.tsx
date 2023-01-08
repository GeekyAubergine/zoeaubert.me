import { useStaticQuery } from 'gatsby'
import { graphql } from 'gatsby'
import { GatsbyImage, getImage } from 'gatsby-plugin-image'
import * as React from 'react'
import { Photo as PhotoType } from '../../../res/photos'

type Props = {
    photo: PhotoType
    fullSize?: boolean
    className?: string
    onClick?: (photo: PhotoType) => void
}

export default function Photo({
    photo,
    className = '',
    onClick,
}: Props) {
    const allImages = useStaticQuery(
        graphql`
            {
                allFile(filter: { dir: { regex: "/images/" } }) {
                    edges {
                        node {
                            relativePath
                            childImageSharp {
                                gatsbyImageData
                            }
                        }
                    }
                }
            }
        `,
    )

    const cleanedPath = photo.path.replace(/^\//, '')

    const imageNode = allImages.allFile.edges.find(
        (edge) => edge.node.relativePath === cleanedPath,
    )

    if (!imageNode) {
        return null
    }

    const image = getImage(imageNode.node)

    if (!image) {
        return null
    }

    const onClickCallback = React.useCallback(() => {
        if (onClick) {
            onClick(photo)
        }
    }, [onClick, photo])

    return (
        <GatsbyImage
            key={photo.path}
            onClick={onClickCallback}
            className={`m-auto ${className} ${
                onClick != null ? 'cursor-pointer' : ''
            }`}
            image={image}
            loading="lazy"
            alt={photo.alt}
        />
    )
}
