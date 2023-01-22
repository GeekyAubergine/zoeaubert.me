async function loadMarkdown() {
    const request = await fetch(
        'https://api.omg.lol/address/geekyaubergine/pastebin/web-now.txt',
    )

    const json = await request.json()

    const { response } = json

    const { paste } = response

    const { content } = paste

    return content
}

module.exports = async function () {
    const [markdown] = await Promise.all([loadMarkdown()])

    return {
        markdown,
    }
}
