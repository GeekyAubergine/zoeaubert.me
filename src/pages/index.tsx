import * as React from 'react'
import { graphql, Link } from 'gatsby'
import * as styles from './index.module.scss'
import BasePage from '../components/page/BasePage'
import BlogPosts from '../components/blog/BlogPosts'

const IndexPage = ({ data }) => {
    return (
        <BasePage>
            <p className={styles.intro}>
                Hi there, I’m a software developer from Jersey, living in
                Portsmouth, working at{' '}
                <a href="https://radweb.co.uk" target="_blank" rel="noopener">
                    Radweb
                </a>{' '}
                on{' '}
                <a
                    href="https://inventorybase.co.uk"
                    target="_blank"
                    rel="noopener"
                >
                    InventoryBase
                </a>{' '}
                and other projects; primarily focusing on app development.
            </p>
            <Link className={styles.blogPostsHeaderLinkWrapper} to="/blog"><p className={styles.blogPostsHeaderLink}>Blog Posts</p></Link>
            <BlogPosts data={data} />
            <Link className={styles.blogPostsLinkWrapper} to="/blog"><p className={styles.blogPostsLink}>Read More</p></Link>

        </BasePage>
    )
}

export const pageQuery = graphql`
    {
        allMarkdownRemark(
            sort: { order: DESC, fields: [frontmatter___date] }
            limit: 2
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

export default IndexPage
