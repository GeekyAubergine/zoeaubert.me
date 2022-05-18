import * as React from 'react'
import { graphql, useStaticQuery } from 'gatsby'
import Helmet from 'react-helmet'

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
                    }
                }
            }
        `,
    )

    return (
        <div className="bg-slate-800 min-h-screen m-w-screen flex justify-center">
            <Helmet
                htmlAttributes={HTML_ATTRIBUTES}
                title={title != null ? `${title} | Zoe Aubert` : 'Zoe Aubert'}
                titleTemplate={`%s`}
                meta={[
                    {
                        name: `description`,
                        content: description,
                    },
                    {
                        property: `og:title`,
                        content: title,
                    },
                    {
                        property: `og:description`,
                        content: description,
                    },
                    {
                        property: `og:type`,
                        content: `website`,
                    },
                    {
                        name: `twitter:card`,
                        content: `summary`,
                    },
                    {
                        name: `twitter:creator`,
                        content: site.siteMetadata.author,
                    },
                    {
                        name: `twitter:title`,
                        content: title,
                    },
                    {
                        name: `twitter:description`,
                        content: description,
                    },
                ]}
            />
            <div className='p-4 max-w-[48em]'>{children}</div>
        </div>
    )
}
