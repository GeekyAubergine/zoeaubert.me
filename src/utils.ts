import {AlbumPhoto} from "../res/photos/albumData";

export const makeAlbumPhotoRemoteUriToLocal = (photo: AlbumPhoto): string => `res/photos/cache/${photo.uid}.jpg`
