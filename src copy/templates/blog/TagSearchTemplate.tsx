import * as React from 'react'
import { graphql } from 'gatsby'
import BasePage from '../../components/page/BasePage'
import BlogPosts from '../../components/blog/BlogPosts'

const BlogPostsPage = ({ data }) => {
    return (
        <BasePage title="Blog" description="Blog">
            <BlogPosts data={data} />
        </BasePage>
    )
}

export default BlogPostsPage
export const pageQuery = graphql`
    query ($tag: String) {
        allMarkdownRemark(
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
