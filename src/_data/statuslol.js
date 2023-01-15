const fetch = require('node-fetch')

async function getLatestStatus() {
    const response = await fetch(
        'https://api.omg.lol/address/geekyaubergine/statuses/',
    )
    const json = await response.json()

    const statuses = json.response.statuses

    const latest = statuses[0]

    console.log({ latest })

    return {
        content: latest.content,
        relativeTime: latest.relative_time,
        link: `https://geekyaubergine.status.lol/${latest.id}`,
    }
}

async function getLatestEmoji() {
    const response = await fetch(
        'https://status.lol/geekyaubergine.js?time&link&fluent',
    )

    const text = await response.text()

    const joinedText = text.replace(/\n/g, ' ')

    const srcMatch = joinedText.match(/src="(.*?)"/)

    const altMatch = joinedText.match(/alt="(.*?)"/)

    if (srcMatch && srcMatch[1] && altMatch && altMatch[1]) {
        return {
            src: srcMatch[1],
            alt: altMatch[1],
        }
    } else {
        throw Error('Unable to find emoji in status.lol response')
    }
}

module.exports = async function () {
    return {
        ...(await getLatestStatus()),
        emoji: await getLatestEmoji(),
    }
}
