const EleventyFetch = require('@11ty/eleventy-fetch')

async function getLatestStatus() {
    const response = await fetch(
        'https://api.omg.lol/address/geekyaubergine/statuses/',
    )
    const json = await response.json()

    const statuses = json.response.statuses

    const latest = statuses[0]

    return {
        content: latest.content,
        relativeTime: latest.relative_time,
        link: `https://geekyaubergine.status.lol/${latest.id}`,
        emoji: latest.emoji,
    }
}

async function getFluentEmoji() {
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

async function getStatuses() {
    const response = await EleventyFetch(
        'https://api.omg.lol/address/geekyaubergine/statuses/',
        {
            duration: '1h',
            type: 'json',
        },
    )

    const {
        response: { statuses },
    } = response

    return statuses.map((status) => ({
        id: status.id,
        url: `https://geekyaubergine.status.lol/${status.id}`,
        date: new Date(status.created * 1000),
        content: status.content,
        emoji: status.emoji,
    }))
}

module.exports = async function () {
    const out = await getStatuses()

    // try {
    //     out.fluentEmoji = await getFluentEmoji()
    // } catch (_e) {
    //     // No fluent emoji, so just ignore
    // }

    // console.log({ out })

    return out
}
