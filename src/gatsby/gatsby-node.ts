import {PHOTO_ALBUM_ALBUMS} from "../../res/photos/albums";
import * as path from "path";

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

const createAlbumPages = ({ createPage }) => {
    const PageComponent = path.resolve('src/templates/AlbumPage.tsx')

    try {
        const albums = Object.keys(PHOTO_ALBUM_ALBUMS)

        albums.forEach((albumKey) => {
            const album = PHOTO_ALBUM_ALBUMS[albumKey]

            if (album != null) {
                createPage({
                    path: `/albums/${album.slug}`,
                    component: PageComponent,
                    context: {
                        albumUid: album.uid,
                    }
                })
            }
        })
    } catch (e) {
        console.error(e)
    }
}

export const createPages = async function ({actions, graphql}) {
    const { createPage } = actions

    try {
        const blogPostsPromise = createBlogPosts({ createPage, graphql })
        const albumPagesPromise = createAlbumPages({ createPage })

        await Promise.all([blogPostsPromise, albumPagesPromise])
    } catch (e) {
        console.error(e)
    }
}