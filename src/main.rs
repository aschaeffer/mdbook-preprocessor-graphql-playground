use anyhow::{bail, Result};
use mdbook::book::Book;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};

fn main() {
    mdbook_preprocessor_boilerplate::run(
        GraphQLPlaygroundPreprocessor,
        "A preprocessor for mdbook to add GraphQL playgrounds.",
    );
}

struct GraphQLPlaygroundPreprocessor;

impl Preprocessor for GraphQLPlaygroundPreprocessor {
    fn name(&self) -> &str {
        "graphql-playground-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        // we *are* a no-op preprocessor after all
        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}
