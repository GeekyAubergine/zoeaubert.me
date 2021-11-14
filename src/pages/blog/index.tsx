import * as React from "react"
import {graphql, Link} from "gatsby"
import * as styles from './blog.module.scss'
import BlogPostPage from "../../components/page/BlogPostPage";

type BlogEntryNode = {
    node: {
        frontmatter: {
            title: string,
            slug: string,
            categories: string[],
            description: string,
            date: string,
        },
        time: number,
    },
}

type QueryResponse = {
    allMarkdownRemark: {
        edges: BlogEntryNode[]
    }
}

type Props = {
    data: QueryResponse,
}

const Category = (category: string) =>
    <p className={styles.category}>{category}</p>

const BlogEntry = ({
   node
}: BlogEntryNode) => {
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
        <BlogPostPage title="Blog" description="Blog" header="Blog Posts">
            <div className={styles.posts}>
                {allMarkdownRemark.edges.map(BlogEntry)}
            </div>
        </BlogPostPage>
    )
}

export default BlogPage

export const pageQuery = graphql`
{
  allMarkdownRemark(sort: {order: DESC, fields: [frontmatter___date]}, limit: 100) {
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