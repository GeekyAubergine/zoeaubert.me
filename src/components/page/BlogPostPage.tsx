import * as React from 'react'
import BasePage from "./BasePage";

type Props = {
    title: string,
    description: string,
    children: React.ReactNode,
}

const BlogPostPage = ({
    title,
    description,
    children
}: Props) => (
    <BasePage title={title} description={description} >
    <h1>{title}</h1>
        {children}
    </BasePage>
)

export default BlogPostPage