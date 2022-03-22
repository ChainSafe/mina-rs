const fs = require('fs')
const path = require('path')
const { exec } = require('child_process');

const OUT_BASE_DIR = path.join(__dirname, 'web', 'pb')

function execPwd(command) {
    exec(command, { cwd: __dirname }, (error, stdout, stderr) => {
        if (stdout) {
            console.log(stdout)
        }
        if (stderr) {
            console.error(stderr)
        }
    })
}

function convertProtos(dir, subDir) {
    const files = fs.readdirSync(path.join(dir, subDir), { withFileTypes: true })
    files.forEach(entry => {
        const fullPath = path.join(dir, subDir, entry.name)
        if (entry.isFile()) {
            if (entry.name.endsWith('.proto')) {
                const outDir = path.join(OUT_BASE_DIR, subDir)
                if (!fs.existsSync(outDir)) {
                    fs.mkdirSync(outDir)
                }
                const outJsPath = path.join(outDir, entry.name.replace('.proto', '.js'))
                const outTypePath = outJsPath.replace('.js', '.d.ts')
                execPwd(`npx pbjs -t static-module --lint eslint-disable -o '${outJsPath}' '${fullPath}'`)
                execPwd(`npx pbts -o '${outTypePath}' '${outJsPath}'`)
            }
        } else {
            convertProtos(dir, path.join(subDir, entry.name))
        }
    })
}

function main() {
    const protoDir = path.join(__dirname, 'proto')
    convertProtos(protoDir, '')
}

main()
