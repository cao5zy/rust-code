#[macro_use]
extern crate json;

extern crate getopts;
use getopts::Options;
use std::env;

fn build_args() -> String {
    let parsed = json::parse(
        r#"
                             {
                             "i":{
                             "long": "input",
                             "desc": "需要解析的文件名称"
                             }
                             }

    "#,
    )
    .unwrap();

    for (key, value) in parsed.entries().rev(){
        dbg!(key);
        dbg!(value["long"]);
    }
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optopt("i", "--input", "需要解析的文件名称", "input");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            panic!("{}", e)
        }
    };

    matches.opt_str("i").unwrap()
}

fn main() {
    use std::io::stderr;
    use swc::config::SourceMapsConfig;
    use swc::ecmascript::ast::ModuleItem;
    use swc::ecmascript::ast::{EsVersion, ImportDecl, ModuleDecl};
    use swc::Compiler;
    use swc_atoms::JsWord;
    use swc_common::sync::Lrc;
    use swc_common::{errors::Handler, FileName, SourceMap, DUMMY_SP};
    use swc_ecma_ast::{ImportSpecifier, Str};
    use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsConfig};

    println!("{}", build_args());

    let source = "
    import React from \"react\";
    import ReactDOM from \"react-dom\";
    import {Button, Input} from \"antd\";
    import Child from \"./component/Child\";

    class Page extends React.Component {
        render() {
            return (
                <div className={\"test\"}>
                    <div>Page</div>
                    <Child/>
                    <Button>click me</Button>
                    <Input/>
                </div>
            );
        }
    }

    ReactDOM.render(<Page/>, document.getElementById(\"root\"));
    ";

    let _source2 = "\
        function abc(){\
            console.log(123);
        }";

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".into()), source.into());

    let compiler = Compiler::new(cm.clone());
    // let _handler = Handler::with_emitter_writer(Box::new(stderr()), Some(compiler.cm.clone()));

    let lexer = Lexer::new(
        // We want to parse ecmascript
        Syntax::Typescript(TsConfig {
            tsx: true,
            decorators: true,
            dynamic_import: true,
            dts: false,
            no_early_errors: false,
            import_assertions: false,
        }),
        // EsVersion defaults to es5
        EsVersion::Es2016,
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let list_error = parser.take_errors();
    if list_error.iter().len() > 0 {
        let mut err_msg = "".to_owned();
        for err in list_error {
            let msg = err.into_kind().msg().to_string();
            err_msg.push_str(msg.as_str());
        }
    }

    let mut module = parser.parse_module().unwrap();

    println!("parser success");

    let s = serde_json::to_string_pretty(&module).expect("failed to serialize");
    // println!("ast json is \n {}", s);
}
