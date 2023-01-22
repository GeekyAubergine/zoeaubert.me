module.exports = async function () {
    const request = await fetch(
        'https://api.omg.lol/address/geekyaubergine/pastebin/web-about.txt',
    )

    const json = await request.json()

    const { response } = json

    const { paste } = response

    const { content } = paste

    return content
}
