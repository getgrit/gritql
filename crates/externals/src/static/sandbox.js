/**
 * This is a very simple sandbox for WebAssembly + Extism.
 * It relies on "eval" to execute the code, but the whole thing *does* run inside a WebAssembly sandbox.
 * See https://extism.org/blog/sandboxing-llm-generated-code/ for more information.
 */

function register_function() {
  let code = Host.inputString();
  Var.set('code', code);
}

function register_parameter_names() {
  let params = Host.inputString();
  Var.set('params', params);
}

function invoke() {
  let code = Var.getString('code');

  let params_encoded = Var.getString('params');
  let params = JSON.parse(params_encoded);

  let eval_code = 'module.exports = function(input) {\n';
  for (let i = 0; i < params.length; i++) {
    // Why use a proxy?
    // Because we want to theoretically support late binding, so we only throw an error if a user accesses an unbound variable
    let name = params[i];
    eval_code +=
      'let ' +
      name +
      ' = new Proxy({}, { get: function (target, prop, receiver) { ' +
      'if (prop !== "text") { throw new Error(`Called ${prop} on ' +
      name +
      ', only text is supported`); }' +
      'return input[' +
      i +
      ']; } });\n';
  }
  eval_code += code;
  eval_code += '\n};';

  // Host.outputString(eval_code);

  let input_encoded = Host.inputString();
  let input = JSON.parse(input_encoded);

  let func = eval(eval_code);
  Host.outputString(func(input).toString());
}

module.exports = { register_parameter_names, register_function, invoke };
