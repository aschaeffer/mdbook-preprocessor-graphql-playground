# mdbook-preprocessor-graphql-playground

[<img src="https://img.shields.io/badge/Docs-mdBook-brightgreen">](https://aschaeffer.github.io/mdbook-preprocessor-graphql-playground/)
[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/mdbook-preprocessor-graphql-playground/Rust">](https://github.com/aschaeffer/mdbook-preprocessor-graphql-playground/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/mdbook-preprocessor-graphql-playground">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/mdbook-preprocessor-graphql-playground">]()

> A preprocessor for mdbook to add GraphQL playgrounds.

[![preview](book/src/images/screenshot.png)](book/src/images/screenshot.png)

## Usage

1. Add to `book.toml`:
    ```toml
    [preprocessor.graphql-playground]
    command = "mdbook-graphql-playground"
    renderer = ["html"]
    
    [output]
    
    [output.html]
    additional-js = ["graphql-playground-react-middleware.js", "graphql-playground.js"]
    additional-css = ["graphql-playground.css"]
    ```
2. Copy assets into the book root directory:
    ```shell
   cp assets/graphql-playground.css <book_dir>
   cp assets/graphql-playground.js <book_dir>
   cp assets/graphql-playground-react-middleware.js <book_dir>
   cp assets/images/* <book_dir>/images/
    ```
3. You may have to adjust the CSS
4. Create query files
5. Create a config file
