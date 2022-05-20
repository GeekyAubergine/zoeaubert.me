import * as React from 'react'
import { Link } from 'gatsby'

export default function BlogListItem({ node, style = '' }) {
    console.log({ node })

    const renderTagCallback = React.useCallback((tag) => {
        console.log({ tag })
        return (
            <Link
                to={`/blog/tags/${tag.toLowerCase()}`}
                className="bg-slate-700 py-1 px-1.5 rounded"
            >
                {tag}
            </Link>
        )
    }, [])

    return (
        <Link
            to={`/blog/${node.frontmatter.slug}`}
            className={`flex flex-col mt-2 mb-6 no-underline sm:my-2 ${style}`}
        >
            <div className="flex items-center">
                <h3 className="text-2xl link">{node.frontmatter.title}</h3>
            </div>
            <p>{node.frontmatter.description}</p>
            <div className="flex flex-row justify-between">
                <div className="flex flex-row">
                    <p className="secondary">{node.frontmatter.date}</p>
                    <p className="secondary mx-1">-</p>
                    <p className="secondary">{node.timeToRead} min</p>
                </div>
                <div>{node.frontmatter.tags.map(renderTagCallback)}</div>
            </div>
        </Link>
    )
}
