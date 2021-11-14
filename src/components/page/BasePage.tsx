import * as React from 'react';
import {graphql, useStaticQuery} from 'gatsby'
import Helmet from 'react-helmet'
import * as styles from './page.module.scss'

const HTML_ATTRIBUTES = {
    lang: 'en',
}

type Props = {
    title: string | null,
    description: string | null,
    children: React.ReactNode,
}

const BasePage = ({title, description, children}: Props) => {
    const {site} = useStaticQuery(
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
        <div className={styles.wrapper}>
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
            <div className={styles.body}>
                {children}
            </div>
        </div>
    )
}

export default BasePage