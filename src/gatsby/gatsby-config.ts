module.exports = {
    siteMetadata: {
        siteUrl: 'https://zoeaubert.me',
        title: 'zoeaubert.me',
        description: '',
        author: 'Zoe Aubert | GeekyAubergine',
    },
    plugins: [
        'gatsby-plugin-sass',
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
            resolve: `gatsby-plugin-google-fonts`,
            options: {
                fonts: [
                    `Nunito:300,300i,400,400i,500,500i,600,600i,700,700i`,
                    `Poppins:300,300i,400,400i,500,500i,600,600i,700,700i`,
                    `Muli:300,300i,400,400i,500,500i,600,600i,700,700i`,
                    `FiraCode:300,300i,400,400i,500,500i,600,600i,700,700i`,
                ],
                display: `swap`,
                subset: `greek-ext,latin-ext`,
            },
        },
        {
            resolve: 'gatsby-plugin-manifest',
            options: {
                name: `gatsby-starter-default`,
                short_name: `starter`,
                start_url: `/`,
                background_color: `#ECBDF2`,
                theme_color: `#ECBDF2`,
                display: `minimal-ui`,
                icon: './res/images/icon.png',
            },
        },
        {
            resolve: 'gatsby-source-filesystem',
            options: {
                name: 'pages',
                path: './src/pages/',
            },
            __key: 'pages',
        },
        {
            resolve: 'gatsby-source-filesystem',
            options: {
                name: 'images',
                path: './res/images/',
            },
            __key: 'images',
        },
        {
            resolve: 'gatsby-source-filesystem',
            options: {
                name: 'blog_posts',
                path: './res/blog_posts/',
            },
            __key: 'blog_posts',
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
                ],
            },
        },
    ],
}
