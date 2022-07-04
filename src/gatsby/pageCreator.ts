import * as path from 'path'
import { PHOTO_ALBUM_ALBUMS } from '../../res/photos/albumData'

export const createBlogPosts = async ({ createPage, graphql, reporter }) => {
    const BlogPost = path.resolve(
        'src/templates/BlogPost.tsx',
    )
    const BlogTags = path.resolve(
        'src/templates/BlogTags.tsx',
    )

    try {
        const result = await graphql(`
            {
                allMarkdownRemark(
                    filter: { fileAbsolutePath: { regex: "/res/blog_posts/" } }
                ) {
                    pageInfo {
                        perPage
                    }
                    edges {
                        node {
                            frontmatter {
                                slug
                            }
                            id
                            timeToRead
                        }
                    }
                }
                tagsGroup: allMarkdownRemark(limit: 2000) {
                  group(field: frontmatter___tags) {
                    fieldValue
                  }
                }
            }
        `)

        if (result.errors) {
            reporter.panicOnBuild(
                `Error while running GraphQL query to build pages`,
            )
            return
        }

        const { data } = result

        const { allMarkdownRemark, tagsGroup } = data

        const { edges } = allMarkdownRemark

        const pages = edges.map(({ node }) => ({
            slug: node.frontmatter.slug,
            id: node.id,
        }))

        pages.forEach(({ slug, id, timeToRead }) => {
            createPage({
                path: `/blog/${slug}`,
                component: BlogPost,
                context: {
                    id,
                    timeToRead,
                },
            })
        })

        const tags = result.data.tagsGroup.group.map(g => Object.values(g)[0])

        tags.forEach((tag) => {
            createPage({
                path: `/blog/tags/${tag.toLowerCase()}`,
                component: BlogTags,
                context: {
                    tag,
                },
            })
        })
    } catch (e) {
        console.error(e)
    }
}
