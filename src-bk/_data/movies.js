const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/movies.json`)

    const json = await request.json()

    return json
}
