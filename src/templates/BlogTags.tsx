import { graphql } from 'gatsby'
import * as React from 'react'
import BlogListItem from '../components/ui/BlogListItem'
import { Page } from '../components/ui/Page'

export default function BlogTags({ data, pageContext }) {
    const renderBlogEntry = React.useCallback(({ node }) => {
        return <BlogListItem node={node} key={node.id} />
    }, [])

    return (
        <Page title="Blog">
            <h2 className="text-2xl pt-12 mb-2 font-bold sm:pt-8">
                {pageContext.tag} Blog Posts
            </h2>
            {data.blogPosts.edges.map(renderBlogEntry)}
        </Page>
    )
}

export const pageQuery = graphql`
    query ($tag: String) {
        blogPosts: allMarkdownRemark(
            sort: { order: DESC, fields: [frontmatter___date] }
            limit: 1000
            filter: {
                fileAbsolutePath: { regex: "/res/blog_posts/" }
                frontmatter: { tags: { in: [$tag] } }
            }
        ) {
            pageInfo {
                perPage
            }
            edges {
                node {
                    frontmatter {
                        title
                        slug
                        tags
                        description
                        date(formatString: "YYYY-MM-DD")
                    }
                    timeToRead
                }
            }
        }
    }
`
