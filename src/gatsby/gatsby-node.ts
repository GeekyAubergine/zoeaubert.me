import { createFilePath } from 'gatsby-source-filesystem'
import { createBlogPosts, createPhotoPages } from './pageCreator'

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
