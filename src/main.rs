use chrono::Local;
use env_logger::Builder;
use log::{error, warn, LevelFilter};
use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use std::collections::HashMap;
use std::env;
use tera::{to_value, Context, Tera, Value};
use uuid::Uuid;

fn main() {
    init_logging();
    mdbook_preprocessor_boilerplate::run(GraphQLPlaygroundPreprocessor, "A preprocessor for mdbook to add GraphQL playgrounds.");
}

struct GraphQLPlaygroundPreprocessor;

impl Preprocessor for GraphQLPlaygroundPreprocessor {
    fn name(&self) -> &str {
        "graphql-playground"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                let context = Context::new();
                chapter.content = match tera().render_str(chapter.content.as_str(), &context) {
                    Ok(rendered) => rendered, // chapter.content.to_string()
                    Err(e) => {
                        warn!("Failed to render chapter \"{}\": {}", chapter.name.as_str(), e);
                        chapter.content.to_string()
                    }
                }
            }
        });
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn graphql_playground(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let id = args.get("id").and_then(|id| id.as_str()).map_or(Uuid::new_v4().to_string(), |id| id.into());
    match args.get("config") {
        Some(config_location) => match to_value(format!(
            "<graphql-playground id=\"{}\" src=\"{}\">\n</graphql-playground>\n",
            id,
            config_location.as_str().unwrap_or("")
        )) {
            Ok(html) => Ok(html),
            Err(e) => {
                error!("Failed to replace: {}", e);
                Err(e.into())
            }
        },
        None => {
            error!("Missing argument 'config'");
            Err("Missing argument 'config'".into())
        }
    }
}

fn tera() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]);
    tera.register_function("graphql_playground", graphql_playground);
    tera
}

fn init_logging() {
    use std::io::Write;
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}) [{}:{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.file().unwrap_or("unknown"),
            record.line().unwrap_or(0),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();
}
