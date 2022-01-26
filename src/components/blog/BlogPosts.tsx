import * as React from 'react'
import { Link } from 'gatsby'
import * as styles from './blogPosts.module.scss'
import BasePage from '../page/BasePage'
import { MarkdownRemarkNode, MarkdownRemarkResponse } from '../../types'

type QueryResponse = {
    allMarkdownRemark: {
        edges: MarkdownRemarkNode[]
    }
}

type Props = {
    data: QueryResponse
}

const Tag = (tag: string) => (
    <Link key={tag} className={styles.tag} to={`/blog/tags/${tag.toLowerCase()}`}>
        {tag}
    </Link>
)

const BlogEntry = ({ node }: MarkdownRemarkNode) => {
    const { frontmatter, timeToRead } = node
    const { title, slug, tags, date, description } = frontmatter

    return (
        <Link key={slug} className={styles.post} to={`/blog/${slug}`}>
            <h2 className={styles.title}>{title}</h2>
            <p className={styles.description}>{description}</p>
            <div className={styles.dateAndTags}>
                <p className={styles.date}>
                    {`${timeToRead}m read`} · {date}
                </p>
                <div className={styles.tags}>{tags.map(Tag)}</div>
            </div>
        </Link>
    )
}

const BlogPosts = ({ data }: Props) => {
    const { allMarkdownRemark } = data
    return (
        <div className={styles.posts}>
            {allMarkdownRemark.edges.map(BlogEntry)}
        </div>
    )
}

export default BlogPosts
