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
    // console.log({ array, item })
    return array.includes(item)
}

function cleanTag(tag) {
    return tag.replace(/ /g, '-').toLowerCase()
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

    eleventyConfig.setServerOptions({
        showAllHosts: true,
    })

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
