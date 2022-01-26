import * as React from 'react'
import { graphql } from 'gatsby'
import BlogPosts from '../../components/blog/BlogPosts'
import BasePage from '../../components/page/BasePage'

const BlogPostsPage = ({ data }) => {
    return (
        <BasePage title="Blog" description="Blog">
            <BlogPosts data={data} />
        </BasePage>
    )
}

export default BlogPostsPage
export const pageQuery = graphql`
    {
        allMarkdownRemark(
            sort: { order: DESC, fields: [frontmatter___date] }
            limit: 1000
            filter: { fileAbsolutePath: { regex: "/res/blog_posts/" } }
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
