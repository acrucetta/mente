# Mente

Welcome to the Mente repository. Mente is a wiki links generator written in Rust. It dynamically processes a set of `.htm` files and generates an interactive, web-based wiki with hypertextual connections.

## How it Works

Mente recursively scans a given directory for `.htm` files and generates a lexicon. Every occurrence of a term between curly braces `{}` in the `.htm` files is considered a wiki link, and the corresponding target file will be linked in the generated HTML output. 

The application also supports linking to subsections via portal templates, which are denoted by a term beginning with a `/` between the curly braces. This causes Mente to include the contents of the linked file directly in the source page, under the linked term as a header.

All the links and their corresponding file references are stored in the lexicon, which helps in checking the existence of the linked files and increments the references to each file when they are used in other files.

Mente also generates an index page listing all the pages in the wiki.

## Usage

To use Mente, you would typically:

1. Create a collection of `.htm` files with content you want to include in your wiki. Place these files in the `src/inc` directory.
2. Use curly braces `{}` in your `.htm` files to create wiki links to other `.htm` files. For example, `{term}` would create a wiki link to `term.htm`.
3. Run the Mente program.

## Running the Program

Mente can be run with the following steps:

1. Clone the repository to your local machine.
2. Make sure you have Rust installed. You can download Rust from [here](https://www.rust-lang.org/tools/install).
3. Navigate to the root directory of the project and run the following command in your terminal:

```bash
cargo run
```

## Testing
Mente also includes tests. To run them, use the following command:

```bash
cargo test
```
## Features

TODO
- Parse markdown with Pandoc
- Add code syntax highlighting
- Improve typography based on gwern.net
- Clean up the deployment pipeline to run cargo run upon committing to main 
- Test showing images from local folder
- Add preview of backlinks in the site
- Code folding-style collapse/disclosure support (both inline & block)
- Automatic link-ification of keywords (LinkAuto.hs)

License
Mente is open-source software licensed under MIT.
