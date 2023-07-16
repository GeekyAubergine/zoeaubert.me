const child_process = require('child_process')

module.exports = function () {
    // https://stackoverflow.com/a/34518749/5323344
    const latestGitCommitHash = child_process
        .execSync('git rev-parse --short HEAD')
        .toString()
        .trim()

    console.log('env', process.env.ENVIRONMENT)

    return {
        buildHash: latestGitCommitHash,
        isProduction: process.env.ENVIRONMENT === 'production',
    }
}
