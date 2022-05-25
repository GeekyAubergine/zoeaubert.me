import { graphql } from 'gatsby'
import * as React from 'react'
import { Page } from '../components/ui/Page'

export default function BlogPost({ data }) {
    const { markdownRemark } = data
    const { frontmatter, html, timeToRead } = markdownRemark
    const { title, date } = frontmatter

    return (
        <Page title={title}>
            <h2 className="text-3xl pt-12 mb-0 font-bold sm:pt-8">{title}</h2>
            <div className="flex flex-row mb-6">
                <p className="secondary">{date}</p>
                <p className="secondary mx-1">-</p>
                <p className="secondary">{timeToRead} min</p>
            </div>
            <div
                className="m-w-full content"
                dangerouslySetInnerHTML={{ __html: html }}
            />
        </Page>
    )
}

export const pageQuery = graphql`
    query ($id: String!) {
        markdownRemark(id: { eq: $id }) {
            html
            frontmatter {
                title
                slug
                description
                date(formatString: "YYYY-MM-DD")
                tags
            }
            timeToRead
        }
    }
`
