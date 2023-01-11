import { createFilePath, createRemoteFileNode } from 'gatsby-source-filesystem'
import { createAlbumNodes } from './nodeCreator'
import { createBlogPosts, createPhotoPages } from './pageCreator'

export const ALBUM_NODE_TYPE = 'Album'
export const ALBUM_PHOTO_NODE_TYPE = 'AlbumPhoto'

export async function createPages({ actions, graphql, reporter }) {
    const { createPage } = actions

    try {
        const blogPostsPromise = createBlogPosts({
            createPage,
            graphql,
            reporter,
        })
        const photosPagesPromise = createPhotoPages({
            createPage,
            graphql,
            reporter,
        })

        await Promise.all([blogPostsPromise, photosPagesPromise])
    } catch (e) {
        console.error(e)
    }
}

export async function sourceNodes(props) {
    await createAlbumNodes(props)
}

export function createSchemaCustomization({ actions }) {
    const { createTypes } = actions
    createTypes(
        `type AlbumPhoto implements Node {
            id: ID!
            albumUid: String!
            url: String!
            description: String
            tags: [String!]
            featured: Boolean
            album: Album @link(by: "uid", from: "albumUid")
            localFile: File @link(from : "fields.localFile")
        }
        type Album implements Node {
            id: ID!
            uid: String!
            title: String!
            date: Date! @dateformat
            description: String
            photos: [AlbumPhoto] @link(by: "albumUid", from: "uid")
        }
        `,
    )
}

export async function onCreateNode({
    node,
    actions,
    getNode,
    getCache,
    createNodeId,
}) {
    console.log('node', node.internal.type)
    const { createNode, createNodeField } = actions
    if (node.internal.type === `MarkdownRemark`) {
        const value = createFilePath({ node, getNode })
        createNodeField({
            name: `slug`,
            node,
            value,
        })
    }
    if (node.internal.type === ALBUM_PHOTO_NODE_TYPE) {
        const fileNode = await createRemoteFileNode({
            url: node.url,
            parentNodeId: node.id,
            createNode,
            createNodeId,
            getCache,
        })
        if (fileNode) {
            createNodeField({
                node,
                name: 'localFile',
                value: fileNode.id,
            })
        }
    }
}
