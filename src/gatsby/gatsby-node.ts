import { createBlogPosts, createPhotoPages } from './pageCreator'

export const createPages = async ({ actions, graphql, reporter }) => {
  const { createPage } = actions

  try {
    const blogPostsPromise = createBlogPosts({ createPage, graphql, reporter })
    const photosPagesPromise = createPhotoPages({ createPage, graphql, reporter })

    await Promise.all([blogPostsPromise, photosPagesPromise])
  } catch (e) {
    console.error(e)
  }
}
