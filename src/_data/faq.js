const config = require('../../config')

module.exports = async function () {
    const { apiUrl } = config

    const request = await fetch(`${apiUrl}/faq.txt`)

    return request.text()
}
