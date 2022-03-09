const fs = require('fs')

const filePath = 'pkg/wasm.js'
let content = fs.readFileSync(filePath, { encoding: 'utf-8' })
content = content
    .replace(/const(.+?)\s*=\s*require\s*\(String\.raw`(.+?)`\).+/g, 'import $1 from "$2"')
    // .replace(/:\s*Text.+?coder\d/g, '')
    .replace('const { Text', '// const { Text')
    .replace('const path = ', '/* const path = ')
content += `
*/
module.exports.init = async function (stream) {
	module.exports.__wasm = wasm;
    const wasmInstance = await (await WebAssembly.instantiateStreaming(stream, imports)).instance;
    wasm = wasmInstance.exports;
    return wasm
}
`
// .replace('const bytes = ', 'const bytes = require("wasm_bg")\nconsole.log("reloading wasm")\n// const bytes =')
// console.log(content)
fs.writeFileSync(filePath, content, { encoding: 'utf-8' })
