import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../../components/ui/Page'
import BlogListItem from '../../components/ui/BlogListItem'

export default function IndexPage({ data }) {
    const renderBlogEntry = React.useCallback(({ node }) => {
        return <BlogListItem node={node} key={node.id} />
    }, [])

    return (
        <Page title="Blog">
            <h2 className="pageTitle">Blog Posts</h2>
            {data.blogPosts.edges.map(renderBlogEntry)}
        </Page>
    )
}

export const pageQuery = graphql`
    {
        blogPosts: allMarkdownRemark(
            sort: { frontmatter: { date: DESC } }
            filter: { fileAbsolutePath: { regex: "/res/blog_posts/" } }
        ) {
            pageInfo {
                perPage
            }
            edges {
                node {
                    id
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
