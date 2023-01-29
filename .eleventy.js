const timeToRead = require('eleventy-plugin-time-to-read')
const Image = require('@11ty/eleventy-img')
const syntaxHighlight = require('@11ty/eleventy-plugin-syntaxhighlight')
const pluginRss = require('@11ty/eleventy-plugin-rss')
const marked = require('marked')

const POSTS_GLOB = './src/_content/posts/**/*.md'
const MB_TAGS_TO_IGNORE = ['selfLink', 'status']
const MB_CONTENT_TO_IGNORE = /https?:\/\/zoeaubert\.me/

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

function arrayIncludesShortcode(array, item) {
    console.log({ array, item })
    return array.includes(item)
}

function cleanTag(tag) {
    return tag.replace(/ /g, '-').toLowerCase()
}

function transformBlogPostToStandardFormat(post) {
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

function transformMicroBlogPostToStandardFormat(post, stripTags = false) {
    return {
        type: 'microBlog',
        url: post.url,
        title: post.title,
        date: new Date(post.date),
        tags: post.tags ?? [],
        content:
            stripTags === true
                ? post.content.replace(/<img.*>/g, '').replace(/<\/?a.*?>/g, '')
                : post.content,
    }
}

function shouldKeepMicroBlogPost(microBlogPost) {
    if (MB_CONTENT_TO_IGNORE.test(microBlogPost.content)) {
        return false
    }

    return MB_TAGS_TO_IGNORE.every((tag) => !microBlogPost.tags.includes(tag))
}

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

function timelineFromCollectionApi(collectionApi) {
    const rawPosts = collectionApi.getFilteredByGlob(POSTS_GLOB)

    const { data } = collectionApi.items[0]

    const posts = rawPosts.map(transformBlogPostToStandardFormat)

    const microblogPosts = data.microblog
        .map(transformMicroBlogPostToStandardFormat)
        .filter(shouldKeepMicroBlogPost)

    const albums = data.albums.albums.map(transformAlbumToStandardFormat)
    const statuses = data.statuslol.map(transformStatusToStandardFormat)

    const firstPost = rawPosts[0]

    const timeline = [...posts, ...albums, ...statuses, ...microblogPosts]

    return timeline.sort((a, b) => new Date(b.date) - new Date(a.date))
}

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

    eleventyConfig.addCollection('timelinePosts', timelineFromCollectionApi)

    eleventyConfig.addCollection('tagList', (collectionApi) => {
        const timeline = timelineFromCollectionApi(collectionApi)

        const tags = timeline.reduce((acc, post) => {
            if (post.type !== 'blogPost' && post.type !== 'microBlog') {
                return acc
            }

            if (!post.tags) {
                return acc
            }

            post.tags.forEach((tag) => {
                const cleaned = cleanTag(tag)
                if (!acc.includes(cleaned)) {
                    acc.push(cleaned)
                }
            })

            return acc
        }, [])

        return tags
    })

    eleventyConfig.addCollection('posts', (collection) =>
        collection
            .getFilteredByGlob(POSTS_GLOB)
            .reverse()
            .map(transformBlogPostToStandardFormat),
    )

    eleventyConfig.addCollection('recentPosts', (collection) =>
        collection
            .getFilteredByGlob(POSTS_GLOB)
            .reverse()
            .slice(0, 5)
            .map(transformBlogPostToStandardFormat),
    )

    eleventyConfig.addCollection('recentMicroblogPosts', (collection) =>
        collection.items[0].data.microblog
            .map(transformMicroBlogPostToStandardFormat)
            .filter(shouldKeepMicroBlogPost)
            .slice(0, 5),
    )

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

    eleventyConfig.addFilter('cleanTag', cleanTag)

    eleventyConfig.addShortcode('renderPhoto', renderPhotoShortcut)
    eleventyConfig.addShortcode('arrayIncludes', arrayIncludesShortcode)

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
