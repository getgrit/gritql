use anyhow::Result;
use extism::{Manifest, Plugin, Wasm};

pub struct ExternalFunction {
    plugin: Plugin,
    name: String,
}

impl ExternalFunction {
    pub fn new_js(js_function_body: &[u8], param_names: Vec<String>) -> Result<Self> {
        let sandbox = include_bytes!("./static/sandbox.wasm");
        let eval_sandbox = Wasm::data(sandbox.to_vec());
        let manifest = Manifest::new([eval_sandbox]);
        // Currently the JS PDK requires WASI to be enabled; this is not really secure
        let mut plugin = Plugin::new(manifest, [], true)?;

        plugin.call::<&[u8], ()>("register_function", js_function_body)?;
        plugin.call::<&[u8], ()>(
            "register_parameter_names",
            serde_json::to_vec(&param_names)?.as_slice(),
        )?;

        Ok(Self {
            plugin,
            name: "invoke".to_string(),
        })
    }

    pub fn call(&mut self, input_bindings: &[&str]) -> Result<Vec<u8>> {
        let serialized = serde_json::to_vec(input_bindings)?;
        let data: &[u8] = self.plugin.call(&self.name, serialized)?;
        Ok(data.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_say_hello() -> Result<()> {
        let js_script = include_bytes!("../fixtures/js/say_hello.js");

        let mut plugin = ExternalFunction::new_js(
            js_script,
            vec!["$greeting".to_string(), "$person".to_string()],
        )
        .unwrap();

        let output = plugin.call(&["Hello", "sam"]).unwrap();
        println!("output: {:?}", String::from_utf8(output.to_vec())?);
        assert_eq!(output, b"Hello, sam");

        Ok(())
    }

    #[test]
    fn test_js_say_hello_string() -> Result<()> {
        let js_script = include_bytes!("../fixtures/js/say_hello.js");

        let mut plugin = ExternalFunction::new_js(
            js_script,
            vec!["$greeting".to_string(), "$person".to_string()],
        )
        .unwrap();

        // Notice we have quotes here
        let output = plugin.call(&["Hello", "\"max\""]).unwrap();
        println!("output: {:?}", String::from_utf8(output.to_vec())?);
        assert_eq!(output, b"Hello, \"max\"");

        Ok(())
    }

    #[test]
    #[ignore = "This currently fails, it seems we can't call the same function twice"]
    fn test_js_say_hello_twice() -> Result<()> {
        let js_script = include_bytes!("../fixtures/js/say_hello.js");

        let mut plugin = ExternalFunction::new_js(
            js_script,
            vec!["$greeting".to_string(), "$person".to_string()],
        )
        .unwrap();

        // Call it the first time
        let output = plugin.call(&["Hello", "sam"]).unwrap();
        assert_eq!(output, b"Hello, sam");

        // Call it with a new set of inputs
        let output = plugin.call(&["Goodbye", "Alex"]).unwrap();
        assert_eq!(output, b"Goodbye, Alex");

        Ok(())
    }
}
