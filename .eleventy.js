const timeToRead = require('eleventy-plugin-time-to-read')
const Image = require('@11ty/eleventy-img')

// https://www.11ty.dev/docs/plugins/image/
async function renderPhotoShortcut(photo, alt, classes = '') {
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
            width="${width}"
            height="${height}"
            loading="lazy"
            decoding="async" />
        `
}

module.exports = function (eleventyConfig) {
    eleventyConfig.addWatchTarget('./albums')

    eleventyConfig.addPlugin(timeToRead)

    eleventyConfig.addPassthroughCopy('./assets')

    eleventyConfig.addCollection('posts', (collection) =>
        collection.getFilteredByGlob('./posts/**/*.md').reverse(),
    )

    eleventyConfig.addCollection('recentPosts', (collection) =>
        collection
            .getFilteredByGlob('./posts/**/*.md')
            .reverse()
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

    eleventyConfig.addFilter(
        'debug',
        (content) => `<pre>${inspect(content)}</pre>`,
    )

    eleventyConfig.addFilter('stripIndex', function (path) {
        return path.replace('/index.html', '/')
    })

    eleventyConfig.addAsyncShortcode('renderPhoto', renderPhotoShortcut)

    eleventyConfig.setWatchThrottleWaitTime(100)

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
