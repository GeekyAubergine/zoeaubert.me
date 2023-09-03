const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/tags.json`)

    const json = await request.json()

    const { postsByTag, allTags, tagCounts } = json

    return {
        postsByTag,
        allTags,
        tagCounts,
    }
}
