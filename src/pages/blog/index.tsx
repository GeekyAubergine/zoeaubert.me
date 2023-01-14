import * as React from 'react'
import { graphql, Link } from 'gatsby'
import { Page } from '../../components/ui/Page'
import BlogListItem from '../../components/ui/BlogListItem'
import SEO from '../../components/Seo'

export default function IndexPage({ data }) {
    const renderBlogEntry = React.useCallback(({ node }) => {
        return <BlogListItem node={node.childMarkdownRemark} key={node.id} />
    }, [])

    return (
        <Page>
            <h2 className="pageTitle">Blog Posts</h2>
            {data.blogPosts.edges.map(renderBlogEntry)}
        </Page>
    )
}

export const Head = () => (
    <SEO title="Blog" description="List of all my blog posts" />
)

export const pageQuery = graphql`
    {
        blogPosts: allFile(
            filter: { sourceInstanceName: { eq: "posts" } }
            sort: { childMarkdownRemark: { frontmatter: { date: DESC } } }
        ) {
            edges {
                node {
                    id
                    childMarkdownRemark {
                        frontmatter {
                            title
                            slug
                            description
                            date(formatString: "YYYY-MM-DD")
                        }
                        id
                        timeToRead
                    }
                }
            }
        }
    }
`
