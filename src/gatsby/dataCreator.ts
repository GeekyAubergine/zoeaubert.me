import {
  PHOTO_ALBUM_ALBUMS,
  PHOTO_ALBUM_PHOTOS,
} from '../../res/photos/albumData'
import { createRemoteFileNode } from 'gatsby-source-filesystem'

const NODE_TYPE_PHOTO = 'Photo'
const NODE_TYPE_ALBUM = 'Album'

export const createCustomNodeSchemas = ({ actions }) => {
  const { createTypes } = actions

  const typeDefs = `
        type Photo implements Node {
          localFile: File @link(from: "fields.localFile")
        }
    `

  createTypes(typeDefs)
}

const createNodeWithId = async (
  type: string,
  id: string,
  nodeData: any,
  { createNodeId, createNode, getNode, createContentDigest },
) => {
  const nodeId = createNodeId(id)

  await createNode({
    id: nodeId,
    parent: null,
    ...nodeData,
    children: [],
    internal: {
      type: type,
      content: JSON.stringify(nodeData),
      contentDigest: createContentDigest(nodeData),
    },
  })

  return getNode(nodeId)
}

export const createAlbumNodes = async ({
  createNodeId,
  createNode,
  createContentDigest,
  getNode,
  reporter,
  store,
  cache,
  createNodeField,
}) => {
  return Promise.all(
    Object.keys(PHOTO_ALBUM_ALBUMS).map(async (albumUid: string) => {
      const album = PHOTO_ALBUM_ALBUMS[albumUid]

      if (album == null) {
        return
      }

      const albumNode = await createNodeWithId(
        NODE_TYPE_ALBUM,
        `${NODE_TYPE_ALBUM}-${album.uid}`,
        album,
        {
          createNodeId,
          createNode,
          createContentDigest,
          getNode,
        },
      )

      await Promise.all(
        album.photo_uids.map(async (photoUid: string) => {
          const photo = PHOTO_ALBUM_PHOTOS[photoUid]

          if (photo == null) {
            return
          }

          const photoNodeData = {
            ...photo,
            albumUid,
          }

          const photoNode = await createNodeWithId(
            NODE_TYPE_PHOTO,
            `${NODE_TYPE_PHOTO}-${photo.uid}`,
            photoNodeData,
            {
              createNodeId,
              createNode,
              createContentDigest,
              getNode,
            },
          )

          const fileNode = await createRemoteFileNode({
            reporter,
            store,
            url: photo.uri,
            parentNodeId: photoNode.id,
            createNode,
            createNodeId,
            cache,
          })

          if (fileNode != null) {
            await createNodeField({
              node: photoNode,
              name: 'localFile',
              value: fileNode.id,
            })
          }
        }),
      )
    }),
  )
}
