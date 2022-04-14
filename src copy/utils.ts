import { AlbumPhoto } from '../res/photos/albumData'
import { PhotoFile } from './types'

export const makeAlbumPhotoRemoteUriToLocal = (photo: AlbumPhoto): string =>
    `res/photos/cache/${photo.uid}.jpg`

export const isPhotoFileLandscape = (photoFile: PhotoFile): boolean =>
    photoFile.childImageSharp.gatsbyImageData.width >
    photoFile.childImageSharp.gatsbyImageData.height

export const isPhotoFilePortrait = (photoFile: PhotoFile): boolean =>
    !isPhotoFileLandscape(photoFile)
