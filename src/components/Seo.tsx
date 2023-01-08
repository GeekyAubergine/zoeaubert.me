import React from 'react'
import { graphql, useStaticQuery } from 'gatsby'
import { Helmet } from 'react-helmet'

const HTML_ATTRIBUTES = {
    lang: 'en',
}

type Props = {
    title?: string | null
    description?: string | null
    image?: string | null
    preventIndexing?: boolean
}

export default function SEO({
    title,
    description,
    image,
    preventIndexing,
}: Props) {
    const { site } = useStaticQuery(
        graphql`
            query {
                site {
                    siteMetadata {
                        title
                        description
                        author
                        siteUrl
                        image
                    }
                }
            }
        `,
    )

    const {
        title: siteTitle,
        description: siteDescription,
        siteUrl,
        image: siteImage,
    } = site.siteMetadata

    const pageTitle = title != null ? `${title} | ${siteTitle}` : siteTitle
    const pageDescription = description != null ? description : siteDescription
    const pageImage =
        image != null && !image.startsWith('http')
            ? `${siteUrl}${image}`
            : siteImage

    return (
        <>
            <Helmet htmlAttributes={HTML_ATTRIBUTES}>
                <title>{title}</title>
                <meta name="title" content={pageTitle} />
                <meta name="description" content={pageDescription} />

                <meta property="og:type" content="website" />
                <meta property="og:url" content={siteUrl} />
                <meta property="og:title" content={pageTitle} />
                <meta property="og:description" content={pageDescription} />
                <meta property="og:image" content={pageImage} />

                <meta property="twitter:card" content="summary_large_image" />
                <meta property="twitter:url" content={siteUrl} />
                <meta property="twitter:title" content={pageTitle} />
                <meta
                    property="twitter:description"
                    content={pageDescription}
                />
                <meta property="twitter:image" content={pageImage} />

                {preventIndexing && <meta name="robots" content="noindex" />}
            </Helmet>
        </>
    )
}
