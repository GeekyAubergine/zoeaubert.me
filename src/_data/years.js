const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/years.json`)

    const json = await request.json()

    const { years, entitiesByYear } = json

    return {
        years,
        entitiesByYear,
    }
}
