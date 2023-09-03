const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/games.json`)

    const json = await request.json()

    return json
}
