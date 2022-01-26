import * as React from 'react'
import { graphql } from 'gatsby'
import { MarkdownRemarkResponse } from '../../types'
import BlogPost from '../../components/blog/BlogPost'
import BasePage from '../../components/page/BasePage'

type QueryResponse = {
  markdownRemark: MarkdownRemarkResponse
}

type Props = {
  data: QueryResponse
}

const BlogPostPage = ({ data }: Props) => {
  const { markdownRemark } = data
  const { frontmatter } = markdownRemark
  const { title } = frontmatter
  return (
    <BasePage title="Blog" description="Blog">
      <BlogPost data={data} />
    </BasePage>
  )
}

export default BlogPostPage

export const pageQuery = graphql`
  query ($id: String!) {
    markdownRemark(id: { eq: $id }) {
      html
      frontmatter {
        title
        slug
        description
        date(formatString: "YYYY-MM-DD")
        tags
      }
      timeToRead
    }
  }
`
