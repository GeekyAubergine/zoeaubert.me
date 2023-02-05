const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config
    
    const request = await fetch(`${apiUrl}/links.txt`)

    return request.text()
}
