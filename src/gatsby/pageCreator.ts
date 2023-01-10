import * as path from 'path'
import {
    ALBUMS,
    albumToSlug,
    ALL_PHOTO_TAGS,
    photoAndAlbumToSlug,
    photoToFileName,
} from '../../res/photos'

export async function createBlogPosts({ createPage, graphql, reporter }) {
    const BlogPost = path.resolve('src/templates/BlogPost.tsx')
    const BlogTags = path.resolve('src/templates/BlogTags.tsx')

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
                tags: allFile(filter: { sourceInstanceName: { eq: "posts" } }) {
                    group(
                        field: {
                            childMarkdownRemark: {
                                frontmatter: { tags: SELECT }
                            }
                        }
                    ) {
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

        const { posts, tags: tagsResult } = data

        const { edges } = posts

        const pages = edges.map(({ node }) => ({
            slug: node.childMarkdownRemark.frontmatter.slug,
            id: node.childMarkdownRemark.id,
        }))

        Promise.all(
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

        const tags = tagsResult.group
            .map((g) => g.fieldValue)
            .filter((t) => t.length > 0)

        Promise.all(
            tags.map(async (tag) => {
                await createPage({
                    path: `/blog/tags/${tag.toLowerCase()}`,
                    component: BlogTags,
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

export async function createPhotoPages({ createPage, graphql, reporter }) {
    const AlbumPage = path.resolve('src/templates/AlbumPage.tsx')
    const PhotoTagPage = path.resolve('src/templates/PhotoTagPage.tsx')
    const PhotoPage = path.resolve('src/templates/PhotoPage.tsx')

    try {
        await Promise.all(
            ALBUMS.map(async (album) => {
                const albumPath = albumToSlug(album)

                try {
                    await createPage({
                        path: albumPath,
                        component: AlbumPage,
                        context: {
                            uuid: album.uuid,
                        },
                    })

                    await Promise.all(
                        album.photos.map(async (photo) => {
                            const fileName = photoToFileName(photo)

                            await createPage({
                                path: photoAndAlbumToSlug(album, photo),
                                component: PhotoPage,
                                context: {
                                    albumUuid: album.uuid,
                                    photoPath: photo.path,
                                    fileName,
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
            ALL_PHOTO_TAGS.map(async (tag) => {
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
