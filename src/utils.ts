import { graphql, useStaticQuery } from 'gatsby'
import { DateTime } from 'luxon'
import { PhotoNodeData } from '../res/photos'
import { Album, Photo } from './types'

const FILE_NAME_REGEX = /([\w,\s-]+)\.[A-Za-z]{3}$/

export function usePhotoNodeData(): PhotoNodeData[] {
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
                            publicURL
                        }
                    }
                }
            }
        `,
    )

    return allImages.allFile.edges.map((edge) => ({
        relativePath: edge.node.relativePath,
        gatsbyImageData: edge.node.childImageSharp.gatsbyImageData,
        publicURL: edge.node.publicURL,
    }))
}

export function isPhotoPortrait(photo: Photo): boolean {
    const { width, height } = photo.localFile.childImageSharp.original
    return height > width
}

export function isPhotoLandscape(photo: Photo): boolean {
    return !isPhotoPortrait(photo)
}

export function albumToSlug(album: { date: string; title: string }): string {
    const date = DateTime.fromISO(album.date)

    return `/photos/${date.year}/${date.month < 10 ? '0' : ''}${
        date.month
    }/${album.title.toLowerCase().replace(/\s/g, '-')}`
}

export function photoToFileName(photo: { url: string }): string {
    const matches = photo.url.match(FILE_NAME_REGEX)

    if (!matches) {
        throw new Error('No file name found')
    }

    const fileName = matches[1]

    if (!fileName) {
        throw new Error('No file name found')
    }

    return fileName
}

export function photoAndAlbumToSlug(
    album: { date: string; title: string },
    photo: { url: string },
): string {
    return `${albumToSlug(album)}/${photoToFileName(photo)}`
}
