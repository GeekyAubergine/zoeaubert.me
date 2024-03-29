const timeToRead = require('eleventy-plugin-time-to-read')
const pluginRss = require('@11ty/eleventy-plugin-rss')
const MarkdownIt = require('markdown-it')
const prism = require('markdown-it-prism')
const { execSync } = require('child_process')

const md = new MarkdownIt({
    html: true,
})

md.use(prism, {})

const config = require('./config')

const POSTS_GLOB = './src/_content/posts/**/*.md'
const MB_TAGS_TO_IGNORE = ['selfLink', 'status']
const MB_CONTENT_TO_IGNORE = /https?:\/\/zoeaubert\.me/

// https://www.11ty.dev/docs/plugins/image/
function renderPhotoShortcut(photo, alt, classes = '') {
    if (!photo) {
        throw new Error('Missing photo')
    }

    const { url, width, height } = photo

    if (alt === undefined) {
        // You bet we throw an error on missing alt (alt="" works okay)
        throw new Error(`Missing \`alt\` on responsiveimage from: ${url}`)
    }

    const url2 = url.startsWith('https://')
        ? url
        : `https://cdn.geekyaubergine.com${url}`

    return `
          <img
            class="${classes}"
            src="${url2}"
            alt="${alt}"
            width="${width}"
            height="${height}"
            loading="lazy"
            decoding="async" />
        `
}

function renderImageShortcut(image, classes = '') {
    if (!image) {
        throw new Error('Missing image')
    }

    const { src, width, height, alt } = image

    if (alt === undefined) {
        // You bet we throw an error on missing alt (alt="" works okay)
        throw new Error(`Missing \`alt\` on responsiveimage from: ${src}`)
    }

    const url2 = src.startsWith('https://')
        ? src
        : `https://cdn.geekyaubergine.com${src}`

    return `
          <img
            class="${classes}"
            src="${url2}"
            alt="${alt}"
            width="${width}"
            height="${height}"
            loading="lazy"
            decoding="async" />
        `
}

function arrayIncludesShortcode(array, item) {
    return array.includes(item)
}

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(timeToRead, {
        style: 'short',
    })
    eleventyConfig.addPlugin(pluginRss)

    eleventyConfig.setQuietMode(true)

    eleventyConfig.addPassthroughCopy('./src/assets')
    eleventyConfig.addPassthroughCopy({
        './src/_content/assets': 'assets',
    })
    eleventyConfig.addPassthroughCopy('robots.txt')

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

    eleventyConfig.addFilter('relativeDate', (d) => {
        const date = new Date(d)
        const now = new Date()
        const diff = now.getTime() - date.getTime()

        const seconds = Math.floor(diff / 1000)

        const years = Math.floor(seconds / 31536000)

        if (years > 0) {
            return `${years} year${years === 1 ? '' : 's'} ago`
        }

        const months = Math.floor(seconds / 2592000)

        if (months > 0) {
            return `${months} month${months === 1 ? '' : 's'} ago`
        }

        const weeks = Math.floor(seconds / 604800)

        if (weeks > 0) {
            return `${weeks} week${weeks === 1 ? '' : 's'} ago`
        }

        const days = Math.floor(seconds / 86400)

        if (days > 0) {
            return `${days} day${days === 1 ? '' : 's'} ago`
        }

        const hours = Math.floor(seconds / 3600)

        if (hours > 0) {
            return `${hours} hour${hours === 1 ? '' : 's'} ago`
        }

        const minutes = Math.floor(seconds / 60)

        if (minutes > 0) {
            return `${minutes} minute${minutes === 1 ? '' : 's'} ago`
        }

        if (seconds < 60) {
            return 'just now'
        }

        return date.toISOString()
    })

    eleventyConfig.addFilter('toActualDate', (date) => {
        return new Date(date)
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
        return md.render(content)
    })

    eleventyConfig.addFilter('prefixCDN', function (slug) {
        return `${config.cdnUrl}${slug}`
    })

    eleventyConfig.addFilter('slug', function (slug) {
        if (slug === 'F1') {
            return 'f1'
        }

        if (slug === 'F2') {
            return 'f2'
        }

        if (slug === 'F3') {
            return 'f3'
        }

        if (slug === 'WIPWednesday') {
            return 'wip-wednesday'
        }

        if (slug.endsWith('GP')) {
            return slug.toLowerCase().replace('gp', '-gp')
        }

        if (slug.toLowerCase() === 'tv') {
            return 'tv'
        }

        return slug
            .replace(
                /([A-Z][a-z]+)|(\d+)/g,
                (letter) => `-${letter.toLowerCase()}`,
            )
            .replace(/^-/, '')
    })

    eleventyConfig.addFilter('spaceTag', function (tag) {
        return tag.replace(/([a-z0-9])([A-Z])/g, '$1 $2')
    })

    eleventyConfig.addFilter('formatNumber', function (number) {
        return parseFloat(number).toLocaleString()
    })

    eleventyConfig.addFilter('albumPhotoToRss', (photo) => {
        if (!photo) {
            throw new Error('Missing photo')
        }

        const { alt, thumbnailLarge } = photo

        const { url, width, height } = thumbnailLarge

        if (alt === undefined) {
            // You bet we throw an error on missing alt (alt="" works okay)
            throw new Error(`Missing \`alt\` on responsiveimage from: ${url}`)
        }

        const url2 = url.startsWith('https://')
            ? url
            : `https://cdn.geekyaubergine.com${url}`

        return `
                  <img
                    src="${url2}"
                    alt="${alt}"
                    width="${width}"
                    height="${height}"
                    loading="lazy"
                    decoding="async" />
                `
    })

    eleventyConfig.addFilter('mediaToRss', (media) => {
        if (!media || media.type !== 'image') {
            throw new Error('Missing photo')
        }

        const { url, width, height, alt } = media

        if (alt === undefined) {
            // You bet we throw an error on missing alt (alt="" works okay)
            throw new Error(`Missing \`alt\` on responsiveimage from: ${url}`)
        }

        const url2 = url.startsWith('https://')
            ? url
            : `https://cdn.geekyaubergine.com${url}`

        return `
              <img
                src="${url2}"
                alt="${alt}"
                width="${width}"
                height="${height}"
                loading="lazy"
                decoding="async" />
            `
    })

    eleventyConfig.addShortcode('renderPhoto', renderPhotoShortcut)
    eleventyConfig.addShortcode('renderImage', renderImageShortcut)
    eleventyConfig.addShortcode('arrayIncludes', arrayIncludesShortcode)

    eleventyConfig.addShortcode('currentTimestamp', () => {
        return new Date().toISOString()
    })

    eleventyConfig.addFilter('toArray', (input) => {
        if (input != null && typeof input === 'string') {
            return input.split(',')
        }

        return input
    })

    eleventyConfig.setBrowserSyncConfig({
        notify: true,
        reloadDelay: 2000,
    })

    eleventyConfig.setWatchThrottleWaitTime(250)

    eleventyConfig.setServerOptions({
        showAllHosts: true,
    })

    eleventyConfig.on('eleventy.after', () => {
        execSync(`npx pagefind --source _site --glob \"**/*.html\"`, {
            encoding: 'utf-8',
        })
    })

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
