const timeToRead = require('eleventy-plugin-time-to-read')
const Image = require('@11ty/eleventy-img')
const utils = require('./src/_utils/generateAlbumData.js')

// https://www.11ty.dev/docs/plugins/image/
async function renderPhotoShortcut(photo) {
    const { alt, image, url } = photo

    if (alt === undefined) {
        // You bet we throw an error on missing alt (alt="" works okay)
        throw new Error(`Missing \`alt\` on responsiveimage from: ${url}`)
    }

    let lowsrc = image.avif[0]
    let highsrc = image.avif[image.avif.length - 1]

    return `<picture>
      ${Object.values(image)
          .map((imageFormat) => {
              return `  <source type="${
                  imageFormat[0].sourceType
              }" srcset="${imageFormat
                  .map((entry) => entry.srcset)
                  .join(', ')}" sizes="100vw">`
          })
          .join('\n')}
        <img
          src="${lowsrc.url}"
          width="${highsrc.width}"
          height="${highsrc.height}"
          alt="${alt}"
          loading="lazy"
          decoding="async">
      </picture>`
}

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

    eleventyConfig.addAsyncShortcode('renderPhoto', renderPhotoShortcut)

    eleventyConfig.setWatchThrottleWaitTime(100)

    return {
        dir: {
            input: './src',
            output: '_site',
        },
    }
}
