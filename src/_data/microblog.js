const EleventyFetch = require('@11ty/eleventy-fetch')
const xml2js = require('xml2js')
const config = require('../../.config.json')
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
    return id.replace(/.*\.(com|blog)/g, '')
}

async function fetchFromMicroBlog() {
    const response = await EleventyFetch(
        'https://micro.blog/posts/geekyaubergine?count=10000',
        {
            duration: '1h',
            type: 'json',
            fetchOptions: {
                Authorization: 'Bearer ' + config.microBlog,
            },
        },
    )

    // const response = await axios.get(
    //     'https://micro.blog/posts/geekyaubergine',
    //     {
    //         headers: {
    //             Authorization: 'Bearer ' + config.microBlog,
    //         },
    //         params: {
    //             count: 0,
    //         },
    //     },
    // )

    // console.log({ response })

    // return []

    // const { data } = response

    const { items } = response

    // console.log({ items: items.slice(0, 10)})

    return items
        .filter((post) => {
            // if (MB_TAGS_TO_IGNORE.some((tag) => post.tags.includes(tag))) {
            //     return false
            // }

            // if (post._microblog.is_conversation) {
            //     return false
            // }

            return true
        })
        .map((post) => ({
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
            duration: '1d',
            type: 'json',
        },
    )

    return response.items.map((post) => ({
        title: post.title !== '' ? post.title : null,
        id: cleanId(post.id),
        url: post.url,
        date: new Date(post.date_published),
        content: post.content_html,
        tags: post.tags,
    }))
}

async function fetchPhotoFromMicroBlog() {
    const response = await EleventyFetch(
        'https://geekyaubergine.com/photos/index.json',
        {
            duration: '1h',
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

module.exports = async function () {
    // const x = await fetch('https://micro.blog/posts/geekyaubergine', {
    //     headers: {
    //         Authorization: 'Bearer ' + config.microBlogKey,
    //     },
    // })

    // const xml = await x.json()

    // const json = xml //await xml2js.parseStringPromise(xml)

    // return json.items.map((post) => ({
    //     title: post.title !== '' ? post.title : null,
    //     id: post.id,
    //     url: post.url,
    //     date: new Date(post.date_published),
    //     content: post.content_html,
    //     tags: post.tags,
    // }))

    // console.log({ json })

    const livePosts = await fetchFromMicroBlog()

    return livePosts

    const archivedPosts = await fetchFromArchive()
    const photoPosts = await fetchPhotoFromMicroBlog()

    const livePostsMap = livePosts.reduce((acc, post) => {
        acc[post.id] = post
        return acc
    }, {})

    const archivedPostsMap = archivedPosts.reduce((acc, post) => {
        acc[post.id] = post
        return acc
    }, {})

    const photoPostsMap = photoPosts.reduce((acc, post) => {
        acc[post.id] = post
        return acc
    }, {})

    const allPosts = { ...archivedPostsMap, ...livePostsMap, ...photoPostsMap }

    // console.log(Object.values(allPosts).slice(0, 5))

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
