const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/timeline.json`)

    const json = await request.json()

    const { entities, entityOrder, entitiesByTag, allTags } = json

    return {
        entities,
        entityOrder,
        entitiesByTag,
        allTags,
    }
}
