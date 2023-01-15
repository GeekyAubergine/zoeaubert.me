import React from 'react'
import { graphql, useStaticQuery } from 'gatsby'

type Props = {
    title?: string | null
    description?: string | null
    image?: string | null
    noIndex?: boolean
}

export default function SEO({
    title,
    description,
    image,
    noIndex,
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

    const pageTitle: string =
        title != null ? `${title} | ${siteTitle}` : siteTitle
    const pageDescription = description != null ? description : siteDescription
    const pageImage =
        image != null && !image.startsWith('http')
            ? `${siteUrl}${image}`
            : siteImage

    return (
        <>
            <title>{pageTitle}</title>
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
            <meta property="twitter:description" content={pageDescription} />
            <meta property="twitter:image" content={pageImage} />

            {noIndex && <meta name="robots" content="noindex" />}
        </>
    )
}
