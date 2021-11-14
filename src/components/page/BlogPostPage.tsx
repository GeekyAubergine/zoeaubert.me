import * as React from 'react'
import BasePage from "./BasePage"
import * as styles from './blog.module.scss'

type Props = {
    title: string,
    description: string,
    header?: string,
    children: React.ReactNode,
}

const BlogPostPage = ({
    title,
    description,
    header,
    children
}: Props) => (
    <BasePage title={title} description={description} >
        <h2 className={styles.title}>{header || title}</h2>
        {children}
    </BasePage>
)

export default BlogPostPage