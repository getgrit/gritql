import init, { matchPattern } from './pkg/marzano_wasm_bindings.js';
import TreeSitter from 'web-tree-sitter';
import { helloWorld } from './pkg/marzano_wasm_bindings.js';

// pub async fn matchPattern(
//     pattern: String,
//     // The paths of the files to match against.
//     paths: Vec < String >,
//     // The contents of the files to match against, in the same order as `paths`.
//     contents: Vec < String >,
//     // Library file names, for the language of the pattern.
//     lib_paths: Vec < String >,
//     // Library file contents, in the same order as `lib_paths`.
//     lib_contents: Vec < String >,
//     // LLM API base
//     llm_api_base: String,
//     // LLM API bearer token
//     llm_api_bearer_token: String,
// )

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
        // await match(pattern, content),
        await helloWorld(),
    );
}

document.addEventListener('DOMContentLoaded', async (event) => {
    document.getElementById('gritButton').addEventListener('click', myMatchPattern);
    await init();
});
