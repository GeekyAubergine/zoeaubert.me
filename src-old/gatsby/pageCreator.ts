import * as path from 'path'
import { albumToSlug, photoAndAlbumToSlug } from '../utils'

export async function createBlogPosts({ createPage, graphql, reporter }) {
    const BlogPost = path.resolve('src/templates/BlogPost.tsx')

    try {
        const result = await graphql(`
            {
                posts: allFile(
                    sort: {
                        childMarkdownRemark: { frontmatter: { date: DESC } }
                    }
                    filter: { sourceInstanceName: { eq: "posts" } }
                ) {
                    edges {
                        node {
                            id
                            childMarkdownRemark {
                                frontmatter {
                                    slug
                                }
                                id
                            }
                        }
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

        const { posts } = data

        const { edges } = posts

        const pages = edges.map(({ node }) => ({
            slug: node.childMarkdownRemark.frontmatter.slug,
            id: node.childMarkdownRemark.id,
        }))

        await Promise.all(
            pages.map(async ({ slug, id }) => {
                await createPage({
                    path: `/blog/${slug}`,
                    component: BlogPost,
                    context: {
                        id,
                    },
                })
            }),
        )
    } catch (e) {
        console.error(e)
    }
}

export async function createPhotoPages({ createPage, graphql, reporter }) {
    const AlbumPage = path.resolve('src/templates/AlbumPage.tsx')
    const PhotoTagPage = path.resolve('src/templates/PhotoTagPage.tsx')
    const PhotoPage = path.resolve('src/templates/PhotoPage.tsx')

    try {
        const result = await graphql(`
            {
                allAlbum {
                    edges {
                        node {
                            id
                            title
                            date
                            photos {
                                id
                                url
                            }
                        }
                    }
                }
                allAlbumPhoto {
                    distinct(field: { tags: SELECT })
                }
            }
        `)

        await Promise.all(
            result.data.allAlbum.edges.map(async ({ node }) => {
                const { id: albumId, title, date, photos } = node
                const albumPath = albumToSlug({ title, date })

                try {
                    await createPage({
                        path: albumPath,
                        component: AlbumPage,
                        context: {
                            id: albumId,
                        },
                    })

                    await Promise.all(
                        photos.map(async (photo) => {
                            await createPage({
                                path: photoAndAlbumToSlug(
                                    { title, date },
                                    photo,
                                ),
                                component: PhotoPage,
                                context: {
                                    albumId: albumId,
                                    photoId: photo.id,
                                },
                            })
                        }),
                    )
                } catch (e) {
                    console.error(e)
                }
            }),
        )

        await Promise.all(
            result.data.allAlbumPhoto.distinct.map(async (tag) => {
                const path = `/photos/tags/${tag
                    .toLowerCase()
                    .replace(/ /g, '-')}`

                await createPage({
                    path,
                    component: PhotoTagPage,
                    context: {
                        tag,
                    },
                })
            }),
        )
    } catch (e) {
        console.error(e)
    }
}
