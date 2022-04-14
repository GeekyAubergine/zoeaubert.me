import * as React from 'react'
import { graphql } from 'gatsby'
import * as styles from './blogPost.module.scss'
import { MarkdownRemarkResponse } from '../../types'

type QueryResponse = {
    markdownRemark: MarkdownRemarkResponse
}

type Props = {
    data: QueryResponse
}

const BlogPost = ({ data }: Props) => {
    const { markdownRemark } = data
    const { frontmatter, html, timeToRead } = markdownRemark
    const { title, tags, date } = frontmatter
    return (
        <>
            <h2 className={styles.pageTitle}>{title}</h2>
            <div
                className={styles.content}
                dangerouslySetInnerHTML={{ __html: html }}
            />
        </>
    )
}

export default BlogPost
