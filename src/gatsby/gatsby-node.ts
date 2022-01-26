import { createAlbumPages, createBlogPosts } from './pageCreator'
import { createAlbumNodes, createCustomNodeSchemas } from './dataCreator'

export const createPages = async ({ actions, graphql, reporter }) => {
  const { createPage } = actions

  try {
    const blogPostsPromise = createBlogPosts({ createPage, graphql, reporter })
    const albumPagesPromise = createAlbumPages({ createPage })

    await Promise.all([blogPostsPromise, albumPagesPromise])
  } catch (e) {
    console.error(e)
  }
}

export const createSchemaCustomization = ({ actions }) =>
  createCustomNodeSchemas({ actions })

export const sourceNodes = async ({
  actions,
  createNodeId,
  createContentDigest,
  cache,
  reporter,
  store,
  getNode,
}) => {
  const { createNode, createNodeField } = actions

  try {
    const albumNodePromise = createAlbumNodes({
      createNodeId,
      getNode,
      createContentDigest,
      createNode,
      reporter,
      store,
      cache,
      createNodeField,
    })

    await Promise.all([albumNodePromise])
  } catch (e) {
    console.error(e)
  }
}
