const path = require('path');

const unwrapObject = (obj) => ({...obj})

const createBlogPosts = async ({ createPage, graphql }) => {
  const PageComponent = path.resolve('src/templates/BlogPostPage.tsx')

  try {
    const { data } = await graphql(`
      {
        allMarkdownRemark(filter: {fileAbsolutePath: {regex: "/res/blog_posts/"}}) {
          pageInfo {
            perPage
          }
          edges {
            node {
              frontmatter {
                slug
              }
              id
            }
          }
        }
      }
    `)
    const { allMarkdownRemark } = data

    const { edges } = allMarkdownRemark

    const pages = edges.map(({ node }) => ({
      slug: node.frontmatter.slug,
      id: node.id,
    }))

    pages.forEach(({ slug, id }) => {
      createPage({
        path: `/blog/${slug}`,
        component: PageComponent,
        context: {
          id,
        }
      })
    })
  } catch (e) {
    console.error(e)
  }
}

exports.createPages = async function ({actions, graphql}) {
  const { createPage } = actions

  try {
    const blogPostsPromise = createBlogPosts({ createPage, graphql })

    await Promise.all([blogPostsPromise])
  } catch (e) {
    console.error(e)
  }
}