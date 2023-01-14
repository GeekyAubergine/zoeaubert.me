import fs from 'fs'
import yaml from 'js-yaml'

export const ALBUM_NODE_TYPE = 'Album'
export const ALBUM_PHOTO_NODE_TYPE = 'AlbumPhoto'

const ALBUMS_DATA_PATH = './albums'

async function getFilesRecursive(path, ext) {
    const files = await fs.promises.readdir(path)
    const result: string[] = []
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

async function photoYmlToPhoto({
    url,
    description,
    tags,
    featured,
    albumUid,
    photoIndex,
}) {
    if (!url) {
        throw new Error('url is required')
    }

    return {
        albumUid,
        url,
        description,
        tags,
        featured: featured || false,
        photoIndex,
    }
}

async function albumYamlToAlbum({ title, date, description, photos }) {
    const uid = `${title}-${date}`.replace(/ /g, '-').toLowerCase()
    return {
        uid,
        title,
        date,
        description,
        year: new Date(date).getFullYear(),
        photos: await Promise.all(
            photos.map((photo, photoIndex) =>
                photoYmlToPhoto({
                    ...photo,
                    albumUid: uid,
                    photoIndex,
                }),
            ),
        ),
    }
}

async function loadAlbumData() {
    const files = await getFilesRecursive(ALBUMS_DATA_PATH, 'yml')

    const albums = await Promise.all(
        files.map(async (path) => {
            const data = fs.promises.readFile(path, 'utf8')
            const yml = yaml.load(await data)
            const album = await albumYamlToAlbum(yml)
            return album
        }),
    )
    return albums
}

export async function createAlbumNodes({
    actions,
    reporter,
    createNodeId,
    createContentDigest,
}) {
    const { createNode } = actions

    try {
        const albums = await loadAlbumData()

        albums.forEach((album) => {
            const nodeContent = JSON.stringify(album)

            const albumNodeId = createNodeId(`${ALBUM_NODE_TYPE}-${album.uid}`)

            const { photos, ...albumData } = album

            photos.forEach((photo) => {
                const nodeContent = JSON.stringify(photo)
                const photoNodeId = createNodeId(
                    `${ALBUM_PHOTO_NODE_TYPE}-${photo.url}`,
                )
                const nodeMeta = {
                    id: photoNodeId,
                    parent: null,
                    children: [],
                    internal: {
                        type: ALBUM_PHOTO_NODE_TYPE,
                        content: nodeContent,
                        contentDigest: createContentDigest(photo),
                    },
                }
                const photoNode = { ...photo, ...nodeMeta }
                createNode(photoNode)
            })

            const albumNodeMeta = {
                id: albumNodeId,
                parent: null,
                children: [],
                internal: {
                    type: ALBUM_NODE_TYPE,
                    content: nodeContent,
                    contentDigest: createContentDigest(albumData),
                },
            }
            const albumNode = { ...albumData, ...albumNodeMeta }
            createNode(albumNode)
        })
    } catch (e) {
        console.error(e)
    }
}
