import init, { matchPattern } from './pkg/marzano_wasm_bindings.js';
import TreeSitter from 'web-tree-sitter';

async function match(pattern, content) {
    return await matchPattern(
        pattern,
        ["playground_pattern"],
        [content],
        [],
        [],
        '',
        '',
    )
}

export async function myMatchPattern() {
    await init();
    let textArea1 = document.getElementById('textArea1').value;
    let textArea2 = document.getElementById('textArea2').value;
    let pattern = textArea1;
    let content = textArea2;
    document.getElementById('resultTextArea').value = JSON.stringify(
        await match(pattern, content),
    );
}

document.addEventListener('DOMContentLoaded', async (event) => {
    document.getElementById('gritButton').addEventListener('click', myMatchPattern);
    await init();
});
