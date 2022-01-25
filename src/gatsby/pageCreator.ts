import * as path from "path"
import {PHOTO_ALBUM_ALBUMS} from "../../res/photos/albumData";

export const createBlogPosts = async ({ createPage, graphql }) => {
    const PageComponent = path.resolve('src/components/blog/BlogPostPage.tsx')

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
                  timeToRead
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

        pages.forEach(({ slug, id, timeToRead }) => {
            createPage({
                path: `/blog/${slug}`,
                component: PageComponent,
                context: {
                    id,
                    timeToRead,
                }
            })
        })
    } catch (e) {
        console.error(e)
    }
}

export const createAlbumPages = ({ createPage }) => {
    const PageComponent = path.resolve('src/components/album/AlbumPage.tsx')

    try {
        const albums = Object.keys(PHOTO_ALBUM_ALBUMS)

        albums.forEach((albumKey) => {
            const album = PHOTO_ALBUM_ALBUMS[albumKey]

            createPage({
                path: `/photos/${album.slug}`,
                component: PageComponent,
                context: {
                    albumUid: album.uid,
                }
            })
        })
    } catch (e) {
        console.error(e)
    }
}