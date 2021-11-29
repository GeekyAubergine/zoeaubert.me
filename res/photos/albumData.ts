export type AlbumPhoto = {
    uid: string,
    uri: string,
    alt: string,
    tags: string[],
    date: string,
}

type AlbumPhotos = {
    [key: string]: AlbumPhoto,
}

type Album = {
    uid: string,
    photo_uids: string[],
    slug: string,
    description: string,
    location: string,
    coords: {
        lat: number,
        lng: number,
    },
    date: string,
}

type AlbumData = {
    [key: string]: Album,
}

export const PHOTO_ALBUM_PHOTOS: AlbumPhotos = {
    'blvnt_the_knife_acapulco-1': {
        uid: 'blvnt_the_knife_acapulco-1',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0349.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-2': {
        uid: 'blvnt_the_knife_acapulco-2',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0360.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-3': {
        uid: 'blvnt_the_knife_acapulco-3',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0388.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-4': {
        uid: 'blvnt_the_knife_acapulco-4',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0406.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-5': {
        uid: 'blvnt_the_knife_acapulco-5',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0414.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-6': {
        uid: 'blvnt_the_knife_acapulco-6',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0459.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-7': {
        uid: 'blvnt_the_knife_acapulco-7',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0510.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-8': {
        uid: 'blvnt_the_knife_acapulco-8',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0630.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-9': {
        uid: 'blvnt_the_knife_acapulco-9',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0633.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-10': {
        uid: 'blvnt_the_knife_acapulco-10',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0697.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-11': {
        uid: 'blvnt_the_knife_acapulco-11',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0749.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-12': {
        uid: 'blvnt_the_knife_acapulco-12',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0821.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-13': {
        uid: 'blvnt_the_knife_acapulco-13',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0870.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-14': {
        uid: 'blvnt_the_knife_acapulco-14',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0875.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-15': {
        uid: 'blvnt_the_knife_acapulco-15',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_0945.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
    'blvnt_the_knife_acapulco-16': {
        uid: 'blvnt_the_knife_acapulco-16',
        uri: 'http://cdn.geekyaubergine.com/2019/10/blvnt_the_knife_acapulco/_MG_1002.jpg',
        alt: 'Lead singer of Blvnt the Knife at the Acapulco',
        tags: [],
        date: '2019-10-27',
    },
}

export const PHOTO_ALBUM_ALBUMS: AlbumData = {
    'blvnt_the_knife_acapulco': {
        uid: 'blvnt_the_knife_acapulco',
        photo_uids: [
            'blvnt_the_knife_acapulco-1',
            'blvnt_the_knife_acapulco-2',
            'blvnt_the_knife_acapulco-3',
            'blvnt_the_knife_acapulco-4',
            'blvnt_the_knife_acapulco-5',
            'blvnt_the_knife_acapulco-6',
            'blvnt_the_knife_acapulco-7',
            'blvnt_the_knife_acapulco-8',
            'blvnt_the_knife_acapulco-9',
            'blvnt_the_knife_acapulco-10',
            'blvnt_the_knife_acapulco-11',
            'blvnt_the_knife_acapulco-12',
            'blvnt_the_knife_acapulco-13',
            'blvnt_the_knife_acapulco-14',
            'blvnt_the_knife_acapulco-15',
        ],
        slug: '2019-10-blvnt-the-knife-acapulco',
        description: 'Blvnt the Knife at the Acapulco',
        location: 'Acapulco',
        coords: {
            lat: 50.7868665,
            lng: -1.0822794,
        },
        date: '2019-10-27'
    }
}
