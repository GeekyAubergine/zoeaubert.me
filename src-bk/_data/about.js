const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config
    
    const request = await fetch(`${apiUrl}/about.md`)

    return request.text()
}
