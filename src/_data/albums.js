const fs = require('fs')
const yaml = require('js-yaml')
const Image = require('@11ty/eleventy-img')

const FILE_NAME_REGEX = /([\w,\s-]+)\.[A-Za-z]{3}$/

const ALBUMS_DATA_PATH = './albums'

const PHOTO_PROCESSING_OPTIONS = {
    // widths: [150, 300, 600, 1200, 'auto'],
    widths: [600, 1200, 'auto'],
    formats: ['jpeg'],
    outputDir: './_site/assets/img/',
    urlPath: '/assets/img/',
    filenameFormat: (id, src, width, format, options) => {
        const name = src.split('/').pop()
        const extension = src.split('.').pop()

        return `${name}-${width}.${extension}`
    },
}

function albumToPermalink(album) {
    const date = album.date.split('-')
    return `/photos/${date[0]}/${date[1].padStart(2, '0')}/${album.title
        .toLowerCase()
        .replace(/ /g, '-')
        .replace(/[^a-z0-9-]/g, '')}/index.html`
}

function photoPermalink(albumPermalink, photo) {
    const matches = photo.url.match(FILE_NAME_REGEX)

    if (!matches) {
        throw new Error('No file name found')
    }

    const fileName = matches[1]

    if (!fileName) {
        throw new Error('No file name found')
    }

    return `${albumPermalink.replace('/index.html', '')}/${fileName}/index.html`
}

async function getFilesRecursive(path, ext) {
    const files = await fs.promises.readdir(path)
    const result = []
    for (const file of files) {
        const filePath = `${path}/${file}`
        const stats = await fs.promises.stat(filePath)
        if (stats.isDirectory()) {
            result.push(...(await getFilesRecursive(filePath, ext)))
        } else if (stats.isFile() && filePath.endsWith(ext)) {
            result.push(filePath)
        }
    }
    return result
}

async function loadYamlFile() {
    const files = await getFilesRecursive(ALBUMS_DATA_PATH, 'yml')

    return Promise.all(
        files.map(async (path) => {
            const data = fs.promises.readFile(path, 'utf8')
            return yaml.load(await data)
        }),
    )
}

async function buildPhoto(photo) {
    const { url, description, alt, tags, featured } = photo

    const image = await Image(url, PHOTO_PROCESSING_OPTIONS)

    return {
        url,
        thumnailSmall: {
            url: image.jpeg[0].url,
            width: image.jpeg[0].width,
            height: image.jpeg[0].height,
        },
        thumnailLarge: {
            url: image.jpeg[1].url,
            width: image.jpeg[1].width,
            height: image.jpeg[1].height,
        },
        original: {
            url: image.jpeg[2].url,
            width: image.jpeg[2].width,
            height: image.jpeg[2].height,
        },
        description,
        alt: alt || description,
        tags: tags || [],
        featured: featured || false,
        orientation:
            image.jpeg[0].width >= image.jpeg[0].height
                ? 'landscape'
                : 'portrait',
        image,
    }
}

async function buildPhotos(photos) {
    const processedPhotos = []

    for (let i = 0; i < photos.length; i++) {
        const photo = photos[i]
        processedPhotos.push(buildPhoto(photo))
    }

    return Promise.all(processedPhotos)
}

function calculateAlbumCover(photos) {
    const featuredPhotos = photos.filter((photo) => photo.featured)
    const otherPhotos = photos.filter((photo) => !photo.featured)

    const featuredPortraitPhotos = featuredPhotos.filter(
        (photo) => photo.orientation === 'portrait',
    )
    const featuredLandscapePhotos = featuredPhotos.filter(
        (photo) => photo.orientation === 'landscape',
    )
    const otherPortraitPhotos = otherPhotos.filter(
        (photo) => photo.orientation === 'portrait',
    )
    const otherLandscapePhotos = otherPhotos.filter(
        (photo) => photo.orientation === 'landscape',
    )

    // If featured landscape, use that
    if (featuredLandscapePhotos[0]) {
        return [featuredLandscapePhotos[0]]
    }

    // If 2 featured portrait, use that
    if (featuredPortraitPhotos[0] && featuredPortraitPhotos[1]) {
        return [featuredPortraitPhotos[0], featuredPortraitPhotos[1]]
    }

    // If 1 featured portrait and 1 other portrait, use that
    if (featuredPortraitPhotos[0] && otherPortraitPhotos[0]) {
        return [featuredPortraitPhotos[0], otherPortraitPhotos[0]]
    }

    // If otherLandscapePhotos, use that
    if (otherLandscapePhotos[0]) {
        return [otherLandscapePhotos[0]]
    }

    // If otherPortraitPhotos, use that
    if (otherPortraitPhotos.length > 0) {
        return otherPortraitPhotos.slice(0, 1)
    }

    return photos[0] != null ? [photos[0]] : []
}

async function buildAlbum(album) {
    const { title, description, date, photos: rawPhotos } = album

    const photosWithImage = await buildPhotos(rawPhotos)

    const albumPermalink = albumToPermalink(album)

    const photos = photosWithImage.map((photo) => ({
        ...photo,
        permalink: photoPermalink(albumPermalink, photo),
    }))

    return {
        title,
        description,
        date,
        permalink: albumPermalink,
        photos,
        cover: calculateAlbumCover(photos),
    }
}

async function buildAlbums() {
    const yamlData = await loadYamlFile()

    const albums = []

    for (let i = 0; i < yamlData.length; i++) {
        const album = yamlData[i]
        console.log(`Processing album ${i + 1} of ${yamlData.length}`)
        albums.push(await buildAlbum(album))
    }

    return albums
}

function buildAlbumsByYear(albums) {
    const albumsByYear = albums.reduce((acc, album) => {
        const year = album.date.slice(0, 4)
        if (!acc[year]) {
            acc[year] = []
        }
        acc[year].push(album)
        return acc
    }, {})

    return Object.entries(albumsByYear)
        .map(([year, albums]) => ({
            year,
            albums: albums.reverse(),
        }))
        .reverse()
}

module.exports = async function () {
    const albums = await buildAlbums()

    const albumsByYear = buildAlbumsByYear(albums)

    const photos = albums.reduce((acc, album) => {
        acc.push(...album.photos)
        return acc
    }, [])

    const tagsCounts = photos.reduce((acc, photo) => {
        photo.tags.forEach((tag) => {
            if (!acc[tag]) {
                acc[tag] = 0
            }
            acc[tag]++
        })
        return acc
    }, {})

    const tags = Object.entries(tagsCounts)
        .map(([tag, count]) => ({
            tag,
            count,
        }))
        .sort((a, b) => b.count - a.count)

    return {
        albums,
        albumsByYear,
        photos,
        tags
    }
}
