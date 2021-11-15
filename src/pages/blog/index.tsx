import * as React from "react"
import {graphql, Link} from "gatsby"
import * as styles from './blog.module.scss'
import BasePage from "../../components/page/BasePage";
import {MarkdownRemarkNode, MarkdownRemarkResponse} from "../../types";

type QueryResponse = {
    allMarkdownRemark: {
        edges: MarkdownRemarkNode[]
    }
}

type Props = {
    data: QueryResponse,
}

const Category = (category: string) =>
    <p key={category} className={styles.category}>{category}</p>

const BlogEntry = ({
   node
}: MarkdownRemarkNode) => {
    const {frontmatter} = node
    const {title, slug, categories, date, description} = frontmatter

    return (
        <Link key={slug} className={styles.post} to={`${slug}`}>
            <h3 className={styles.title}>{title}</h3>
            <p className={styles.description}>{description}</p>
            <div className={styles.dateAndCategories}>
                <p className={styles.date}>{date}</p>
                <div className={styles.categories}>
                    {categories.map(Category)}
                </div>
            </div>
        </Link>
    )
}

const BlogPage = ({
   data,
}: Props) => {
    const {allMarkdownRemark} = data
    return (
        <BasePage title="Blog" description="Blog">
            <h2 className={styles.pageTitle}>Blog Posts</h2>
            <div className={styles.posts}>
                {allMarkdownRemark.edges.map(BlogEntry)}
            </div>
        </BasePage>
    )
}

export default BlogPage

export const pageQuery = graphql`
{
  allMarkdownRemark(
    sort: {order: DESC, fields: [frontmatter___date]}
    limit: 100
    filter: {fileAbsolutePath: {regex: "/res/blog_posts/"}}
  ) {
    pageInfo {
      perPage
    }
    edges {
      node {
        frontmatter {
          title
          slug
          categories
          description
          date(formatString: "YYYY-MM-DD")
        }
        timeToRead
      }
    }
  }
}
`