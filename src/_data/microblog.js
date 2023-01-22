const xml2js = require('xml2js')

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
    const response = await fetch('https://geekyaubergine.com/feed.xml')

    const xml = await response.text()

    const json = await xml2js.parseStringPromise(xml)

    const { rss } = json

    const { channel } = rss

    const { item } = channel[0]

    return item.slice(0, POSTS_LIMIT).map((post) => ({
        title: post.title[0] !== '' ? post.title[0] : null,
        url: post.link[0],
        date: new Date(post.pubDate[0]).toISOString().split('T')[0],
        content: post.description[0]
            .replace(/<img.*>/g, '')
            .replace(/<\/?a.*?>/g, ''),
    }))
}
