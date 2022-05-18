import * as React from 'react'
import { Link } from 'gatsby'

export default function BlogListItem({ node }) {
    return (
        <Link
            to={`/blog/${node.frontmatter.slug}`}
            className="flex flex-col m-4 no-underline"
        >
            <div className="flex items-center">
                <h3 className="text-2xl">{node.frontmatter.title}</h3>
            </div>
            <p>{node.frontmatter.description}</p>
            <p className="secondary">{node.frontmatter.date}</p>
        </Link>
    )
}
