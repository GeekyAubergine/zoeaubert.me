type Photo = {
    uid: string,
    path: string,
    width: number,
    height: number,
    alt: string,
    tags: string[],
    date: string,
}

type Photos = {
    [key: string]: Photo,
}

type Album = {
    uid: string,
    photos: string[],
    slug: string,
    description: string,
    location: string,
    coords: {
        lat: number,
        lng: number,
    },
}

type Albums = {
    [key: string]: Album,
}

export const PHOTO_ALBUM_PHOTOS: Photos = {
    'blvnt_the_knife_acapulco-1': {
        uid: 'test1',
        path: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0349.jpg',
        width: 3,
        height: 2,
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    }
}

export const PHOTO_ALBUM_ALBUMS: Albums = {
    'blvnt_the_knife_acapulco': {
        uid: 'blvnt_the_knife_acapulco',
        photos: ['blvnt_the_knife_acapulco-1'],
        slug: '2019-10-blvnt-the-knife-acapulco',
        description: 'Blvnt the Knife at the Acapulco',
        location: 'Acapulco',
        coords: {
            lat: 50.7868665,
            lng: -1.0822794,
        }
    }
}
