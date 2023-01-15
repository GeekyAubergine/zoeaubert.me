module.exports = function (eleventyConfig) {
    eleventyConfig.addPassthroughCopy('./src/assets')

    eleventyConfig.addCollection('posts', (collection) =>
        collection.getFilteredByGlob('./src/posts/*.md'),
    )

    eleventyConfig.addCollection('albums', (collection) =>
        collection.getFilteredByGlob('./src/albums/*.yml'),
    )

    eleventyConfig.addFilter('linkifyMarkdown', (text) => {
        console.log({ text })
        return text.replace(
            /\[(.*?)\]\((.*?)\)/g,
            '<a href="$2" target="_blank" rel="noopener">$1</a>',
        )
    })

    eleventyConfig.setWatchThrottleWaitTime(100)

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
