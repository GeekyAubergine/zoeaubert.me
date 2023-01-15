const fetch = require('node-fetch')

const POSTS_LIMIT = 5

function cleanContent(title, content) {
    return (
        content
            // Replace paragraphs with line breaks
            .replace(/<\/p>/g, '\n')
            .replace(/<[^>]*>?/gm, '')
    )
}

module.exports = async function () {
    const response = await fetch('https://geekyaubergine.com/feed.json')

    const json = await response.json()

    const { items } = json

    return items.slice(0, POSTS_LIMIT).map((item) => {
        const { id, url, content_html, date_published, title = null } = item

        return {
            id,
            url,
            title,
            summary: cleanContent(title, content_html),
            date: date_published.replace(/T.*/, ''),
        }
    })
}
