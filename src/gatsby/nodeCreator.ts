import fs from 'fs'
import yaml from 'js-yaml'
import path from 'path'
import { ALBUM_NODE_TYPE, ALBUM_PHOTO_NODE_TYPE } from './gatsby-node'

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
    albumUid,
}) {
    const dir = path.dirname(albumDataAbsolutePath)

    if (!fileName) {
        throw new Error('fileName is required')
    }

    return {
        albumUid,
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
    const uid = `${title}-${date}`.replace(/ /g, '-').toLowerCase()
    return {
        uid,
        title,
        date,
        description,
        photos: await Promise.all(
            photos.map((photo) =>
                photoYmlToPhoto({
                    ...photo,
                    albumDataAbsolutePath,
                    albumUid: uid,
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

            const { photos, ...albumData } = album

            const photoNodeIds: string[] = []
            photos.forEach((photo) => {
                console.log({ photo })
                const nodeContent = JSON.stringify(photo)
                const photoNodeId = createNodeId(
                    `${ALBUM_PHOTO_NODE_TYPE}-${photo.fileName}`,
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
                photoNodeIds.push(photoNodeId)
            })

            const albumNodeMeta = {
                id: albumNodeId,
                parent: null,
                children: photoNodeIds,
                internal: {
                    type: ALBUM_NODE_TYPE,
                    content: nodeContent,
                    contentDigest: createContentDigest(albumData),
                },
            }
            const albumNode = { ...albumData, ...albumNodeMeta }

            console.log(albumNode)

            createNode(albumNode)
        })

        console.log(albums)
    } catch (e) {
        console.error(e)
    }
}
