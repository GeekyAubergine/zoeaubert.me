const EleventyFetch = require('@11ty/eleventy-fetch')
const xml2js = require('xml2js')
const axios = require('axios')

const POSTS_LIMIT = 5

const MB_TAGS_TO_IGNORE = ['selfLink']

function cleanContent(title, content) {
    return (
        content
            // Replace paragraphs with line breaks
            .replace(/<\/p>/g, '\n')
            .replace(/<[^>]*>?/gm, '')
    )
}

function cleanId(id) {
    return id.replace(/.*?\.(com|blog)/g, '')
}

async function fetchFromMicroBlog() {
    const response = await EleventyFetch(
        'https://geekyaubergine.com/feed.json',
        {
            duration: '6h',
            type: 'json',
        },
    )

    const { items } = response

    return items.map((post) => ({
        title: post.title !== '' ? post.title : null,
        id: cleanId(post.id),
        url: post.url,
        date: new Date(post.date_published),
        content: post.content_html,
        tags: post.tags,
    }))
}

async function fetchFromArchive() {
    const response = await EleventyFetch(
        'https://raw.githubusercontent.com/GeekyAubergine/archive.geekyaubergine.com/main/feed.json',
        {
            duration: '6h',
            type: 'json',
        },
    )

    return response.items.map((post) => ({
        title: post.title !== '' ? post.title : null,
        id: cleanId(post.id),
        url: post.url,
        date: new Date(post.date_published),
        content: post.content_text.replace(
            /src="uploads/g,
            'src="https://cdn.uploads.micro.blog/67943/',
        ),
        tags: post.tags,
    }))
}

module.exports = async function () {
    const livePosts = await fetchFromMicroBlog()
    const archivedPosts = await fetchFromArchive()
    // const photoPosts = await fetchPhotoFromMicroBlog()

    // console.log(archivedPosts)

    const livePostsMap = livePosts.reduce((acc, post) => {
        acc[post.id] = post
        return acc
    }, {})

    const archivedPostsMap = archivedPosts.reduce((acc, post) => {
        acc[post.id] = post
        return acc
    }, {})

    // const photoPostsMap = photoPosts.reduce((acc, post) => {
    //     acc[post.id] = post
    //     return acc
    // }, {})

    // const allPosts = { ...archivedPostsMap, ...livePostsMap, ...photoPostsMap }
    const allPosts = { ...archivedPostsMap, ...livePostsMap }

    return Object.values(allPosts)

    // return livePosts

    // const response = await fetch('https://geekyaubergine.com/feed.json')

    // const json = await response.json()

    // const { items } = json

    // console.log({ items })

    // // const json = await xml2js.parseStringPromise(xml)

    // // const { rss } = json

    // // const { channel } = rss

    // // const { item } = channel[0]

    // return []

    // console.log({ mb: item[0] })

    // return item.map((post) => ({
    //     title: post.title[0] !== '' ? post.title[0] : null,
    //     url: post.link[0],
    //     date: new Date(post.pubDate[0]),
    //     content: post.description[0],
    // }))
}
