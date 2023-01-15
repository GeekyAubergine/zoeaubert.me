const timeToRead = require('eleventy-plugin-time-to-read')

module.exports = function (eleventyConfig) {
    eleventyConfig.addPlugin(timeToRead)

    eleventyConfig.addPassthroughCopy('./src/assets')

    eleventyConfig.addCollection('posts', (collection) =>
        collection.getFilteredByGlob('./src/posts/**/*.md').reverse(),
    )

    eleventyConfig.addCollection('recentPosts', (collection) =>
        collection
            .getFilteredByGlob('./src/posts/**/*.md')
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

    eleventyConfig.setWatchThrottleWaitTime(100)

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
