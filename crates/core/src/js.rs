use swc_common::{SourceMap, FileName};
use swc_ecma_parser::{Parser, StringInput, Syntax};
use swc_ecma_codegen::{Emitter, text_writer::JsWriter};

pub fn format(code: &str) -> Result<String, String> {
    let cm = SourceMap::default();
    let fm = cm.new_source_file(FileName::Custom("input.js".into()), code.into());

    let mut parser = Parser::new(
        Syntax::Es(Default::default()),
        StringInput::from(&*fm),
        None,
    );

    let module = parser.parse_module().map_err(|e| e.to_string())?;

    let mut buf = Vec::new();
    let mut emitter = Emitter {
        cfg: Default::default(),
        cm,
        comments: None,
        wr: JsWriter::new(SourceMap::default(), "\n", &mut buf, None),
    };

    emitter.emit_module(&module).unwrap();
    Ok(String::from_utf8(buf).unwrap())
}
