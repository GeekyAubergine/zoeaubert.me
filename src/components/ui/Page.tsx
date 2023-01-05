import * as React from 'react'
import { graphql, useStaticQuery } from 'gatsby'
import Helmet from 'react-helmet'
import NavBar from './NavBar'
import Footer from './Footer'

const HTML_ATTRIBUTES = {
    lang: 'en',
}

type Props = {
    title?: string | null
    description?: string | null
    children: React.ReactNode
}
export function Page({ title, description, children }: Props) {
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

    const pageTitle = title != null ? `${title} | Zoe Aubert` : 'Zoe Aubert'
    const pageDescription =
        description != null ? description : site.siteMetadata.description

    return (
        <>
            <main className="flex w-full justify-center pt-4 pb-8 px-4 sm:px-8 sm:pt-8">
                <Helmet
                    htmlAttributes={HTML_ATTRIBUTES}
                    title={pageTitle}
                    titleTemplate={`%s`}
                    meta={[
                        {
                            name: 'title',
                            content: pageTitle,
                        },
                        {
                            name: 'description',
                            content: pageDescription,
                        },
                        {
                            property: 'og:type',
                            content: 'website',
                        },
                        {
                            property: 'og:url',
                            content: site.siteMetadata.siteUrl,
                        },
                        {
                            property: 'og:title',
                            content: pageTitle,
                        },
                        {
                            property: 'og:description',
                            content: pageDescription,
                        },
                        {
                            property: 'og:image',
                            content: site.siteMetadata.image,
                        },
                        {
                            property: 'twitter:card',
                            content: 'summary_large_image',
                        },
                        {
                            property: 'twitter:url',
                            content: site.siteMetadata.siteUrl,
                        },
                        {
                            property: 'twitter:title',
                            content: pageTitle,
                        },
                        {
                            property: 'twitter:description',
                            content: pageDescription,
                        },
                        {
                            property: 'twitter:image',
                            content: site.siteMetadata.image,
                        },
                    ]}
                />
                <div className="flex flex-col width-control">
                    <NavBar />
                    {children}
                    <Footer />
                </div>
            </main>
        </>
    )
}
