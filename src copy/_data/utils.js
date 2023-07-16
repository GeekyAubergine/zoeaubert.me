const fs = require('fs')

async function getFilesRecursive(path, ext) {
    const files = await fs.promises.readdir(path)
    const result = []
    for (const file of files) {
        const filePath = `${path}/${file}`
        const stats = await fs.promises.stat(filePath)
        if (stats.isDirectory()) {
            result.push(...(await getFilesRecursive(filePath, ext)))
        } else if (stats.isFile() && filePath.endsWith(ext)) {
            result.push(filePath)
        }
    }
    return result
}

module.exports = {
    getFilesRecursive,
}
