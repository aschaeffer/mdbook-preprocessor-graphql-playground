use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use std::collections::HashMap;
use tera::{to_value, Context, Tera, Value};

fn main() {
    mdbook_preprocessor_boilerplate::run(
        GraphQLPlaygroundPreprocessor,
        "A preprocessor for mdbook to add GraphQL playgrounds.",
    );
}

struct GraphQLPlaygroundPreprocessor;

impl Preprocessor for GraphQLPlaygroundPreprocessor {
    fn name(&self) -> &str {
        "graphql-playground"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> anyhow::Result<Book> {
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                let mut tera = Tera::default();
                tera.register_function("graphql_playground", graphql_playground);
                let context = Context::new();
                chapter.content = match tera.render_str(&chapter.content, &context) {
                    Ok(rendered) => rendered,
                    Err(e) => e.to_string(),
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
    match args.get("id") {
        Some(id) => match args.get("config") {
            Some(config_location) => match to_value(format!(
                "<graphql-playground id=\"{}\" src=\"{}\" />",
                id.as_str().unwrap_or(""),
                config_location
            )) {
                Ok(x) => Ok(x),
                Err(_) => Err("".into()),
            },
            None => Err("".into()),
        },
        None => Err("".into()),
    }
}
