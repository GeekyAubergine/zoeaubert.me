import { createFilePath } from 'gatsby-source-filesystem'
import { createAlbumNodes } from './nodeCreator'
import { createBlogPosts, createPhotoPages } from './pageCreator'

export const ALBUM_NODE_TYPE = 'Album'
export const ALBUM_PHOTO_NODE_TYPE = 'AlbumPhoto'

export const createPages = async ({ actions, graphql, reporter }) => {
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

export const sourceNodes = async (props) => {
    await createAlbumNodes(props)
}

export const createSchemaCustomization = ({ actions }) => {
    const { createTypes } = actions
    createTypes(
        `type AlbumPhoto implements Node {
            id: ID!
            albumUid: String!
            url: String!
            fileName: String!
            description: String
            tags: [String!]
            featured: Boolean
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

export function onCreateNode({ node, actions, getNode }) {
    const { createNodeField } = actions
    if (node.internal.type === `MarkdownRemark`) {
        const value = createFilePath({ node, getNode })
        createNodeField({
            name: `slug`,
            node,
            value,
        })
    }
}
