module.exports = {
    siteMetadata: {
        siteUrl: 'https://zoeaubert.me',
        title: 'zoeaubert.me',
        description:
            'Software developer, musician, photographer, nerd, and some other things. Making computers go bleep bloop since 2010.',
        author: 'Zoe Aubert | GeekyAubergine',
        image: '/web-bg.png',
    },
    plugins: [
        {
            resolve: `gatsby-plugin-postcss`,
            options: {
                cssLoaderOptions: {
                    camelCase: false,
                },
            },
        },
        {
            resolve: `gatsby-omni-font-loader`,
            options: {
                enableListener: true,
                preconnect: [`https://fonts.bunny.net`],
                web: [
                    {
                        name: `Atkinson Hyperlegible`,
                        file: `https://fonts.bunny.net/css?family=atkinson-hyperlegible:400,400i,700,700i&display=swap`,
                    },
                ],
            },
        },
        {
            resolve: `gatsby-plugin-sharp`,
            options: {
                defaults: {
                    formats: [`auto`, `jpg`, `webp`, `avif`],
                    placeholder: `blurred`,
                    quality: 50,
                    breakpoints: [250, 500, 750, 1080, 1366, 1920],
                    backgroundColor: `transparent`,
                    tracedSVGOptions: {},
                    blurredOptions: {},
                    jpgOptions: {},
                    pngOptions: {},
                    webpOptions: {},
                    avifOptions: {},
                },
            },
        },
        `gatsby-transformer-sharp`,
        'gatsby-plugin-image',
        'gatsby-plugin-react-helmet',
        'gatsby-plugin-sitemap',
        'gatsby-plugin-robots-txt',
        'gatsby-plugin-mdx',
        'gatsby-plugin-sharp',
        {
            resolve: 'gatsby-plugin-manifest',
            options: {
                name: `gatsby-starter-default`,
                short_name: `starter`,
                start_url: `/`,
                background_color: `#202022`,
                theme_color: `#FEB847`,
                display: `minimal-ui`,
                icon: '/icon.png',
            },
        },
        {
            resolve: 'gatsby-source-filesystem',
            options: {
                name: 'pages',
                path: './src/pages',
            },
            __key: 'pages',
        },
        {
            resolve: 'gatsby-source-filesystem',
            options: {
                name: 'blog_posts',
                path: './res/blog_posts',
            },
            __key: 'blog_posts',
        },
        {
            resolve: 'gatsby-source-filesystem',
            options: {
                path: './res/images',
            },
        },
        {
            resolve: `gatsby-transformer-remark`,
            options: {
                plugins: [
                    {
                        resolve: `gatsby-remark-autolink-headers`,
                        options: {
                            offsetY: `100`,
                            // icon: `<svg aria-hidden="true" height="40" version="1.1" viewBox="0 0 16 16" width="40"><path fill-rule="evenodd" d="M4 9h1v1H4c-1.5 0-3-1.69-3-3.5S2.55 3 4 3h4c1.45 0 3 1.69 3 3.5 0 1.41-.91 2.72-2 3.25V8.59c.58-.45 1-1.27 1-2.09C10 5.22 8.98 4 8 4H4c-.98 0-2 1.22-2 2.5S3 9 4 9zm9-3h-1v1h1c1 0 2 1.22 2 2.5S13.98 12 13 12H9c-.98 0-2-1.22-2-2.5 0-.83.42-1.64 1-2.09V6.25c-1.09.53-2 1.84-2 3.25C6 11.31 7.55 13 9 13h4c1.45 0 3-1.69 3-3.5S14.5 6 13 6z"></path></svg>`,
                            className: `header-link-icon`,
                            removeAccents: true,
                            isIconAfterHeader: false,
                        },
                    },
                    {
                        resolve: `gatsby-remark-prismjs`,
                        options: {
                            classPrefix: 'language-',
                            inlineCodeMarker: null,
                            aliases: {},
                            showLineNumbers: true,
                            noInlineHighlight: false,
                            languageExtensions: [
                                {
                                    language: 'superscript',
                                    extend: 'javascript',
                                    definition: {
                                        superscript_types: /(SuperType)/,
                                    },
                                    insertBefore: {
                                        function: {
                                            superscript_keywords:
                                                /(superif|superelse)/,
                                        },
                                    },
                                },
                            ],
                            // Customize the prompt used in shell output
                            // Values below are default
                            prompt: {
                                user: 'root',
                                host: 'localhost',
                                global: false,
                            },
                            // By default the HTML entities <>&'" are escaped.
                            // Add additional HTML escapes by providing a mapping
                            // of HTML entities and their escape value IE: { '}': '&#123;' }
                            escapeEntities: {},
                        },
                    },
                    {
                        resolve: `gatsby-remark-images`,
                        options: {
                            maxWidth: 1200,
                        },
                    },
                ],
            },
        },
        {
            resolve: 'gatsby-plugin-fathom',
            options: {
                trackingUrl: 'learned-laugh.zoeaubert.me',
                siteId: 'XPKVFMEO',
                honorDnt: true,
            },
        },
        `gatsby-plugin-fontawesome-css`,
        {
            resolve: `gatsby-plugin-feed`,
            options: {
                query: `
                {
                  site {
                    siteMetadata {
                      title
                      description
                      siteUrl
                      site_url: siteUrl
                    }
                  }
                }
              `,
                feeds: [
                    {
                        serialize: ({ query: { site, allMarkdownRemark } }) => {
                            return allMarkdownRemark.edges.map((edge) => {
                                return Object.assign(
                                    {},
                                    edge.node.frontmatter,
                                    {
                                        description: edge.node.excerpt,
                                        date: edge.node.frontmatter.date,
                                        url:
                                            site.siteMetadata.siteUrl +
                                            edge.node.fields.slug,
                                        guid:
                                            site.siteMetadata.siteUrl +
                                            edge.node.fields.slug,
                                        custom_elements: [
                                            {
                                                'content:encoded':
                                                    edge.node.html,
                                            },
                                        ],
                                    },
                                )
                            })
                        },
                        query: `
                        {
                            allMarkdownRemark(
                                filter: { fileAbsolutePath: { regex: "/res/blog_posts/" } }
                            ) {
                                pageInfo {
                                    perPage
                                }
                                edges {
                                    node {
                                        excerpt
                                        html
                                        fields { slug }
                                        frontmatter {
                                          title
                                          date
                                        }
                                      }
                                }
                            }
                            tagsGroup: allMarkdownRemark(limit: 2000) {
                                group(field: frontmatter___tags) {
                                    fieldValue
                                }
                            }
                        }
                  `,
                        output: '/rss.xml',
                        title: 'Zoe Aubert',
                    },
                ],
            },
        },
    ],
}
