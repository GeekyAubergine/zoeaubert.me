const timeToRead = require('eleventy-plugin-time-to-read')
const Image = require('@11ty/eleventy-img')
const syntaxHighlight = require('@11ty/eleventy-plugin-syntaxhighlight')
const pluginRss = require('@11ty/eleventy-plugin-rss')
const marked = require('marked')

const POSTS_GLOB = './src/_content/posts/**/*.md'
const MICROS_GLOB = './src/_content/micros/**/**/**/*.md'
const MB_TAGS_TO_IGNORE = ['selfLink']

// https://www.11ty.dev/docs/plugins/image/
function renderPhotoShortcut(photo, alt, classes = '') {
    const { url, width, height } = photo

    if (alt === undefined) {
        // You bet we throw an error on missing alt (alt="" works okay)
        throw new Error(`Missing \`alt\` on responsiveimage from: ${url}`)
    }

    return `
          <img
            class="${classes}"
            src="${url}"
            alt="${alt}"
            loading="lazy"
            decoding="async" />
        `
}

const transformBlogPostToStandardFormat = (post) => {
    // console.log('d': post.data)

    return {
        type: 'blogPost',
        url: post.data.permalink,
        title: post.data.title,
        date: post.data.date,
        tags: post.data.tags,
        description: post.data.description,
        content: () => post.content,
        headerImage: null,
    }
}

const transformMicroBlogPostToStandardFormat = (post, stripTags = false) => ({
    type: 'microBlog',
    url: post.url,
    title: post.title,
    date: new Date(post.date),
    tags: post.tags,
    description: post.description,
    content:
        stripTags === true
            ? () =>
                  post.content.replace(/<img.*>/g, '').replace(/<\/?a.*?>/g, '')
            : () => post.content,
    headerImage: null,
})

const transformAlbumToStandardFormat = (album) => ({
    type: 'album',
    url: album.permalink,
    title: album.title,
    date: new Date(album.date),
    tags: album.tags,
    headerImage: album.cover,
    description: album.description,
    content: () => null,
})

const transformStatusToStandardFormat = (status) => ({
    type: 'status',
    ...status,
})

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(timeToRead, {
        style: 'short',
    })
    eleventyConfig.addPlugin(syntaxHighlight)
    eleventyConfig.addPlugin(pluginRss)

    eleventyConfig.setQuietMode(true)

    eleventyConfig.addWatchTarget('./albums')

    eleventyConfig.addPassthroughCopy('./src/assets')
    eleventyConfig.addPassthroughCopy({
        './src/_content/assets': 'assets',
    })

    eleventyConfig.addCollection('posts', (collection) =>
        collection.getFilteredByGlob(POSTS_GLOB).reverse(),
    )

    eleventyConfig.addCollection('recentPosts', (collection) =>
        collection.getFilteredByGlob(POSTS_GLOB).reverse().slice(0, 5),
    )

    // eleventyConfig.addCollection('recentMicroblogPosts', (collection) =>
    //     collection.items[0].data.microblog.slice(0, 5),
    // )

    eleventyConfig.addCollection('timelinePosts', function (collectionApi) {
        const rawPosts = collectionApi.getFilteredByGlob(POSTS_GLOB)
        const rawMicros = collectionApi.getFilteredByGlob(MICROS_GLOB)

        console.log({ rawPosts })

        const { data } = collectionApi.items[0]

        // console.log({ rawMicros })

        const posts = rawPosts.map(transformBlogPostToStandardFormat)
        const micros = rawMicros.map(transformMicroBlogPostToStandardFormat)

        // const microblogPosts = data.microblog
        //     .filter((post) => post)
        //     .map(transformMicroBlogPostToStandardFormat)

        const albums = data.albums.albums.map(transformAlbumToStandardFormat)
        const statuses = data.statuslol.map(transformStatusToStandardFormat)

        // console.log({ c: collectionApi.allGlobalData() })

        const firstPost = rawPosts[0]

        // const timeline = [...microblogPosts]
        // const timeline = [...micros]
        // const timeline = [...posts, ...albums]
        const timeline = [...posts, ...albums, ...statuses]
        // const timeline = [...posts, ...micros, ...albums]

        // console.log({ x })
        return timeline.sort((a, b) => new Date(b.date) - new Date(a.date))
    })

    eleventyConfig.addFilter('linkifyMarkdown', (text) => {
        return text.replace(
            /\[(.*?)\]\((.*?)\)/g,
            '<a href="$2" target="_blank" rel="noopener">$1</a>',
        )
    })

    eleventyConfig.addFilter('formatDate', (date) => {
        const d = new Date(date)
        return `${d.getFullYear()}-${(d.getMonth() + 1)
            .toString()
            .padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`
    })

    eleventyConfig.addFilter('formatDateTime', (date) => {
        const d = new Date(date)

        const ymd = `${d.getFullYear()}-${(d.getMonth() + 1)
            .toString()
            .padStart(2, '0')}-${d.getDate().toString().padStart(2, '0')}`

        const hm = `${d.getHours().toString().padStart(2, '0')}:${d
            .getMinutes()
            .toString()
            .padStart(2, '0')}`

        if (
            d.getMinutes() === 0 &&
            (d.getHours() === 0 || d.getHours() === 1)
        ) {
            return ymd
        }

        return `${ymd} ${hm}`
    })

    eleventyConfig.addFilter('debug', (...args) => {
        console.log(...args)
        debugger
    })

    eleventyConfig.addFilter('stripIndex', function (path) {
        return path.replace('/index.html', '/')
    })

    eleventyConfig.addFilter('mbFilePathToPermalink', function (path) {
        return path
            .replace(/.*_content\/micros\//, '')
            .replace('.md', '/index.html')
    })

    eleventyConfig.addFilter('mdToHtml', function (content = '') {
        return marked.parse(content)
    })

    eleventyConfig.addShortcode('renderPhoto', renderPhotoShortcut)

    eleventyConfig.addShortcode('currentTimestamp', () => {
        return new Date().toISOString()
    })

    eleventyConfig.setWatchThrottleWaitTime(100)

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
