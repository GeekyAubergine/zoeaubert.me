const config = require('../../config')

const RECENT_POSTS_COUNT = 5

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/blog-posts.json`)

    const json = await request.json()

    const { entities, entityOrder } = json

    return {
        posts: entities,
        postOrder: entityOrder,
        recentPostsOrder: entityOrder.slice(0, RECENT_POSTS_COUNT),
    }
}
