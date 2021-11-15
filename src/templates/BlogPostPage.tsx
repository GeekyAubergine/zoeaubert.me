import * as React from 'react'
import {graphql} from "gatsby";
import BasePage from "../components/page/BasePage";
import * as styles from './blogPostPage.module.scss'
import {MarkdownRemarkResponse} from "../types";

type QueryResponse = {
    markdownRemark: MarkdownRemarkResponse,
}

type Props = {
    data: QueryResponse,
}


const BlogPostPage = ({ data }: Props) => {
    const { markdownRemark } = data
    const { frontmatter, html, timeToRead } = markdownRemark
    const { title, categories, date } = frontmatter
    return (
        <BasePage title="Blog" description="Blog">
            <h2 className={styles.pageTitle}>{title}</h2>
            <div className={styles.content} dangerouslySetInnerHTML={{ __html: html }}/>
        </BasePage>
    )
}

export default BlogPostPage


export const pageQuery = graphql`
  query($id: String!) {
    markdownRemark(id: { eq: $id }) {
      html
      frontmatter {
        title
        slug
        description
        date(formatString: "YYYY-MM-DD")
        categories
      }
      timeToRead
    }
  }
`