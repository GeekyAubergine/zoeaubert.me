import { createBlogPosts } from './pageCreator'

export const createPages = async ({ actions, graphql, reporter }) => {
  const { createPage } = actions

  try {
    const blogPostsPromise = createBlogPosts({ createPage, graphql, reporter })

    await Promise.all([blogPostsPromise])
  } catch (e) {
    console.error(e)
  }
}
