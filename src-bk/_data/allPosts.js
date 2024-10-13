const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/all-posts.json`)

    const json = await request.json()

    const { entities, entityOrder } = json

    return {
        posts: entities,
        postOrder: entityOrder,
    }
}
