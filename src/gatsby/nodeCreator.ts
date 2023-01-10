import fs from 'fs'
import yaml from 'js-yaml'
import path from 'path'

const ALBUM_NODE_TYPE = 'Album'
const ALBUM_PHOTO_NODE_TYPE = 'AlbumPhoto'
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
    fileName,
    description,
    tags,
    featured,
    albumDataAbsolutePath,
}) {
    const dir = path.dirname(albumDataAbsolutePath)
    return {
        url: `${dir}/${fileName}`,
        fileName,
        description,
        tags,
        featured: featured || false,
    }
}

async function albumYamlToAlbum({
    title,
    date,
    description,
    photos,
    albumDataAbsolutePath,
}) {
    return {
        uid: `${title}-${date}`.replace(/ /g, '-').toLowerCase(),
        title,
        date,
        description,
        photos: await Promise.all(
            photos.map((photo) =>
                photoYmlToPhoto({ ...photo, albumDataAbsolutePath }),
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
            const album = await albumYamlToAlbum({
                ...yml,
                albumDataAbsolutePath: path,
            })
            return album
        }),
    )
    return albums
}

export async function createAlbumNodes({
    actions,
    graphql,
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

            const photoNodeIds: string[] = []
            album.photos.forEach((photo) => {
                const nodeContent = JSON.stringify(photo)
                const photoNodeId = createNodeId(
                    `${ALBUM_PHOTO_NODE_TYPE}-${photo.fileName}`,
                )
                const nodeMeta = {
                    id: photoNodeId,
                    parent: albumNodeId,
                    children: [],
                    internal: {
                        type: ALBUM_PHOTO_NODE_TYPE,
                        content: nodeContent,
                        contentDigest: createContentDigest(photo),
                    },
                }
                const photoNode = { ...photo, ...nodeMeta }
                createNode(photoNode)
                photoNodeIds.push(photoNodeId)
            })

            const albumNodeMeta = {
                id: albumNodeId,
                parent: null,
                children: photoNodeIds,
                internal: {
                    type: ALBUM_NODE_TYPE,
                    content: nodeContent,
                    contentDigest: createContentDigest(album),
                },
            }
            const albumNode = { ...album, ...albumNodeMeta }

            console.log(albumNode)

            createNode(albumNode)
        })

        console.log(albums)
    } catch (e) {
        console.error(e)
    }
}
