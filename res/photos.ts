import { Console } from 'console'
import { DateTime } from 'luxon'

export const PHOTO_CDN_URL = 'https://cdn.geekyaubergine.com'

export type Photo = {
    url: string
    alt: string
    tags: string[]
    takenAt: string
    featured?: boolean
}

export type Album = {
    uuid: string
    title: string
    description?: string
    photos: Photo[]
    date: string
}

export type Albums = Albums[]

const FARLINGTON_MARSHES_202205: Album = {
    uuid: '8172872f-19b5-4110-b55e-891b1d56d690',
    title: 'Farlington Marshes',
    description:
        'Little Egret, Blacktail Godwit, Avocet, Rock Pipet or Skylark (I think the Pipet is more likely), Black-headed Gull and a chill Cow.\n\nThe Little Egret is easily some of my favourite photos I’ve taken.',
    photos: [
        {
            url: '/2022/04/20220415-8B5A6085.jpg',
            alt: 'Little Egret in shallow sea water catching a crab',
            tags: [
                'birds',
                'farlingon-marches',
                'little-egret',
                'egret',
                'crab',
                'portsmouth',
            ],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A6086.jpg',
            alt: 'Little Egret in shallow sea water catching a crab',
            tags: [
                'birds',
                'farlingon-marches',
                'little-egret',
                'egret',
                'crab',
                'portsmouth',
            ],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A6087.jpg',
            alt: 'Little Egret in shallow sea water catching a crab',
            tags: [
                'birds',
                'farlingon-marches',
                'little-egret',
                'egret',
                'crab',
                'portsmouth',
            ],
            takenAt: '2020-04-15T12:00:00.000Z',
            featured: true,
        },
        {
            url: '/2022/04/20220415-8B5A4918.jpg',
            alt: 'Backtail Godwit swimming through a pond',
            tags: ['birds', 'farlingon-marches', 'godwit', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5115.jpg',
            alt: 'Avercet swimming through a pond',
            tags: ['birds', 'farlingon-marches', 'avercet', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5403.jpg',
            alt: 'Rock Pipet or Skylark flying against clear sky',
            tags: ['birds', 'farlingon-marches', 'rock-pipet', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5546.jpg',
            alt: 'Black-headed Gull flying infront of pond and tall yellow grass',
            tags: [
                'birds',
                'farlingon-marches',
                'back-headed-gull',
                'portsmouth',
            ],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A4811.jpg',
            alt: 'Cow sitting in field infront of tall yellow grass',
            tags: ['cow', 'farlingon-marches', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5552.jpg',
            alt: 'Black-headed Gull flying infront of tall yellow grass',
            tags: [
                'birds',
                'farlingon-marches',
                'black-headed-gull',
                'portsmouth',
            ],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
    ],
    date: '2022-04-15',
}

const ELYSIAN_FIRE_201910: Album = {
    uuid: '3b258ef2-8fee-4248-a6be-eacdc0356a5c',
    title: 'Elysian Fire at the Acapulco',
    date: '2019-10-24',
    photos: [
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0011.jpg',
            alt: 'Elysian Fire singer playing guitar infront of microphone',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0194.jpg',
            alt: 'Elysian Fire bass player',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0156.jpg',
            alt: 'Elysian Fire guitar player',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
            featured: true,
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0098.jpg',
            alt: 'Elysian Fire singer playing guitar infront of microphone',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0083.jpg',
            alt: 'Elysian Fire guitar player',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0149.jpg',
            alt: 'Elysian Fire singer playing guitar infront of microphone',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
            featured: true,
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0229.jpg',
            alt: 'Elysian Fire singer playing guitar infront of microphone',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/elysian_fire_acapulco/20191024-_MG_0326.jpg',
            alt: 'Group photo of Elysian Fire after the show',
            tags: [
                'elysian-fire',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
    ],
}

const BLVNT_THE_KNIFE_201910: Album = {
    uuid: 'c875ddba-19ae-4ffe-8195-251f71dae217',
    title: 'Blvnt the Knife at the Acapulco',
    date: '2019-10-24',
    photos: [
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0349.jpg',
            alt: 'Blvnt the Knife singer infront of microphone',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_1029.jpg',
            alt: 'Blvnt the Knife guitarist singing and pointing at audience',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0630.jpg',
            alt: 'Blvnt the Knife drummer',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0875.jpg',
            alt: 'Blvnt the Knife singer and guitarists playing with audience in frame',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0697.jpg',
            alt: 'Blvnt the Knife singer holding microphone with hands crossed over chest',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0749.jpg',
            alt: 'Blvnt the Knife guitarist holding hands together thanking audience',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_1004.jpg',
            alt: 'Blvnt the Knife guitarist smiling on stage while interacting with audience member',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
            featured: true,
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_1048.jpg',
            alt: 'Blvnt the Knife guitarist singing and holding microphone',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0633.jpg',
            alt: 'Blvnt the Knife drummer',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_0821.jpg',
            alt: 'Blvnt the Knife guitarist playing guitar',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
        {
            url: '/2019/10/blvnt_the_knife_acapulco/20191024-_MG_1132.jpg',
            alt: 'Blvnt the Knife guitarist singing and holding microphone',
            tags: [
                'blvnt-the-knife',
                'acapulco',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-24T12:00:00.000Z',
        },
    ],
}

const BUSKING_FOR_MISFITS_201910: Album = {
    uuid: 'ffb267b3-7c9c-4a19-ac90-a60c454f9995',
    title: 'Busking for Misfits at the Guildhall Village',
    date: '2019-10-19',
    photos: [
        {
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0111.jpg',
            alt: 'Busking for Misfits drummer smiling',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
            featured: true,
        },
        {
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0216.jpg',
            alt: 'Busking for Misfits singer',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },
        {
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0228.jpg',
            alt: 'Busking for Misfits singer playing guitar with pick in mouth',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },{
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0285.jpg',
            alt: 'Busking for Misfits guitarist',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },{
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0315.jpg',
            alt: 'Busking for Misfits singer playing guitar with drummer in the background',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },{
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0327.jpg',
            alt: 'Busking for Misfits guitarist singing',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },{
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0346.jpg',
            alt: 'Busking for Misfits singer waving arms over head',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },
        {
            url: '/2019/10/busking_for_misfits_guildhall_village/20191019-_MG_0402.jpg',
            alt: 'Busking for Misfits drummer playing',
            tags: [
                'busking-for-misfits',
                'guildhall-village',
                'portrait',
                'portsmouth',
                'live-music',
            ],
            takenAt: '2019-10-19T12:00:00.000Z',
        },
        
    ],
}

export const ALBUMS = [
    FARLINGTON_MARSHES_202205,
    BLVNT_THE_KNIFE_201910,
    ELYSIAN_FIRE_201910,
    BUSKING_FOR_MISFITS_201910,
]

export const ALBUMS_BY_DATE = ALBUMS.sort(
    (a, b) =>
        DateTime.fromISO(b.date).toMillis() -
        DateTime.fromISO(a.date).toMillis(),
)

export const ALL_PHOTO_TAGS = ALBUMS.reduce((acc: string[], album) => {
    const out = acc.slice()

    album.photos.forEach((photo) => {
        photo.tags.forEach((tag) => {
            if (!out.includes(tag)) {
                out.push(tag)
            }
        })
    })

    return out
}, [])

export const ALBUMS_BY_UUID = ALBUMS.reduce(
    (acc: { [uuid: string]: Album }, album) => ({
        ...acc,
        [album.uuid]: album,
    }),
    {},
)

export function albumToSlug(album: Album): string {
    const date = DateTime.fromISO(album.date)

    return `/photos/${date.year}/${date.month < 10 ? '0' : ''}${
        date.month
    }/${album.title.toLowerCase().replace(/\s/g, '-')}`
}
