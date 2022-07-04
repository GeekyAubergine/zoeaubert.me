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
    title: string
    description?: string
    photos: Photo[]
    date: string
}

export type Albums = Albums[]

const FarlingonMarches202205: Album = {
    title: 'Farlingon Marches April 2022',
    description: 'Farlington Marshes - Little Egret, Blacktail Godwit, Avocet, Rock Pipet or Skylark (I think the Pipet is more likely), Black-headed Gull and a chill Cow.\n\nThe Little Egret is easily some of my favourite photos I’ve taken.',
    photos: [
        {
            url: '/2022/04/20220415-8B5A6085.jpg',
            alt: 'Little Egret in shallow sea water catching a crab',
            tags: [
                'birds',
                'farlingon_marches',
                'little_egret',
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
                'farlingon_marches',
                'little_egret',
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
                'farlingon_marches',
                'little_egret',
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
            tags: ['birds', 'farlingon_marches', 'godwit', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5115.jpg',
            alt: 'Avercet swimming through a pond',
            tags: ['birds', 'farlingon_marches', 'avercet', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5403.jpg',
            alt: 'Rock Pipet or Skylark flying against clear sky',
            tags: ['birds', 'farlingon_marches', 'rock_pipet', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5546.jpg',
            alt: 'Black-headed Gull flying infront of pond and tall yellow grass',
            tags: [
                'birds',
                'farlingon_marches',
                'backheaded_gull',
                'portsmouth',
            ],
            takenAt: '2020-04-15T12:00:00.000Z',
            featured: true,
        },
        {
            url: '/2022/04/20220415-8B5A4811.jpg',
            alt: 'Cow sitting in field infront of tall yellow grass',
            tags: ['cow', 'farlingon_marches', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
        {
            url: '/2022/04/20220415-8B5A5552.jpg',
            alt: 'Black-headed Gull flying infront of tall yellow grass',
            tags: ['birds', 'farlingon_marches', 'godwit', 'portsmouth'],
            takenAt: '2020-04-15T12:00:00.000Z',
        },
    ],
    date: '2022-04-15',
}

export const ALBUMS = [FarlingonMarches202205]

export const ALBUMS_BY_DATE = ALBUMS.sort(
    (a, b) =>
        DateTime.fromISO(a.date).toMillis() -
        DateTime.fromISO(b.date).toMillis(),
)

export function albumToSlug(album: Album): string {
    const date = DateTime.fromISO(album.date)
    return `/photos/${date.year}/${date.month < 10 ? '0' : ''}${date.month}/${album.title
        .toLowerCase()
        .replace(/\s/g, '-')}`
}
