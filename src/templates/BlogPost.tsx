import { graphql } from 'gatsby'
import * as React from 'react'
import SEO from '../components/Seo'
import { Page } from '../components/ui/Page'

export default function BlogPost({ data }) {
    const { markdownRemark } = data
    const { frontmatter, html, timeToRead } = markdownRemark
    const { title, date } = frontmatter

    return (
        <Page>
            <h2 className="pageTitle mb-1">{title}</h2>
            <div className="flex flex-row mb-4">
                <time className="text secondary" dateTime={date}>
                    {date}
                </time>
                <p className="secondary mx-1">-</p>
                <p className="secondary">{timeToRead} min</p>
            </div>
            <div
                className="w-full content"
                dangerouslySetInnerHTML={{ __html: html }}
            />
        </Page>
    )
}

export function Head({ data }) {
    const { markdownRemark } = data
    const { frontmatter } = markdownRemark
    const { title, description } = frontmatter

    return <SEO title={title} description={description} />
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
