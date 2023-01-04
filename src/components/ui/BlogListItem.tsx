import * as React from 'react'
import { Link } from 'gatsby'

export default function BlogListItem({ node, style = '' }) {
    return (
        <div
            className={`flex flex-col my-1 no-underline sm:my-2 ${style} text`}
        >
            <Link
                to={`/blog/${node.frontmatter.slug}`}
                className={`flex flex-col no-underline ${style} text`}
            >
                <div className="flex items-center">
                    <h3 className="text-lg link font-normal mb-1">
                        {node.frontmatter.title}
                    </h3>
                </div>
            </Link>
            <p className="text">{node.frontmatter.description}</p>
            <div className="flex flex-row justify-between flex-wrap mt-2">
                <div className="flex flex-row">
                    <p className="secondary">{node.frontmatter.date}</p>
                    <p className="secondary mx-1">-</p>
                    <p className="secondary">{node.timeToRead} min</p>
                </div>
            </div>
        </div>
    )
}
