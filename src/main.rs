use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use chrono::{Local};

const NAME: &str = "Mente";
const DOMAIN: &str = "andrescn.me/mente";
const SOURCE: &str = "https://github.com/acrucetta/wiki-links/edit/main";

#[derive(Debug)]
struct Lexicon {
    files: HashMap<String, usize>,
}

impl Lexicon {
    fn new() -> Lexicon {
        Lexicon {
            files: HashMap::new(),
        }
    }

    fn index(&mut self, dir: &Path) -> Result<(), std::io::Error> {
        let entries = fs::read_dir(Path::new(dir))?;
        for entry in entries {
            let entry = entry?;
            let filename = entry.file_name().into_string().unwrap();
            if filename.ends_with(".htm") {
                self.files.insert(filename, 0);
            }
        }

        Ok(())
    }

    fn find_file(&self, filename: &str) -> Option<&usize> {
        self.files.get(filename)
    }

    fn increment_ref(&mut self, filename: &str) {
        if let Some(value) = self.files.get_mut(filename) {
            *value += 1;
        }
    }
}

fn add_connection(file: &mut File, lex: &mut Lexicon, filename: &str) -> io::Result<()> {
    let std_filename = filename.replace(' ', "_").to_lowercase();
    let htm_filename = format!("{}.htm", std_filename);

    // Check if it's a template of {/template} type
    // if so we will add the title plus its content
    if let Some((0, '/')) = filename.char_indices().next() {
        let trimmed_filename = filename.trim_start_matches('/');
        add_portal(file, lex, trimmed_filename, true);
    }

    match lex.find_file(&htm_filename) {
        Some(_) => {
            let link = format!(
                "<a href='{}.html' class='local'>{}</a>",
                std_filename, filename
            );
            lex.increment_ref(&htm_filename);
            write!(file, "{}", link)
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Missing link: {}", filename),
        )),
    }
}

fn add_main_body(file: &mut File, lex: &mut Lexicon, source_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(source_path)?;
    let mut temp = String::new();
    let mut in_template = false;

    for c in content.chars() {
        match c {
            '{' => {
                in_template = true;
            }
            '}' => {
                in_template = false;
                add_connection(file, lex, &temp)?;
                temp.clear();
            }
            _ if in_template => {
                temp.push(c);
            }
            _ => {
                write!(file, "{}", c)?;
            }
        }
    }
    Ok(())
}

fn add_portal(file: &mut File, lex: &mut Lexicon, s: &str, head: bool) -> io::Result<()> {
    let filename = s.replace(' ', "_").to_lowercase();
    let relative_filepath = format!("src/inc/{}.htm", filename);
    let raw_filepath = format!("{}.htm", filename);

    match lex.find_file(&raw_filepath) {
        Some(_) => {
            let page_content = fs::read_to_string(relative_filepath)?;
            if head {
                writeln!(
                    file,
                    "<h2 id='{}'><a href='{}.html'>{}</a></h2>",
                    filename, filename, s
                )?;
            }
            write!(file, "{}", page_content)?;

            // Increase the reference count for the target page
            lex.increment_ref(&filename);
            Ok(())
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Missing portal {}", s),
        )),
    }
}

fn create_index_page(lex: &Lexicon) -> Result<(), std::io::Error> {
    let mut index = String::new();
    index.push_str("<html><head><title>Index</title></head><body>");
    index.push_str("<ul class='col2 capital'>");

    // We want to index the files in alphabetical order
    let mut filenames = lex.files.keys().cloned().collect::<Vec<String>>();
    filenames.sort();

    for filename in filenames {
        // If the file is an index.htm file, skip it
        if filename == "index.htm" || filename == "meta.nav.htm" {
            continue;
        }
        let name_without_extension = filename.trim_end_matches(".htm");
        index.push_str(&format!(
            "<li><a href=\"{}.html\">{}</a></li>",
            name_without_extension,
            name_without_extension.replace('_', " ")
        ));
    }
    index.push_str("</ul></body></html>");
    fs::write("src/inc/index.htm", index)
}

fn generate_pages(lex: &mut Lexicon) -> std::io::Result<()> {
    let filenames = &lex.files.keys().cloned().collect::<Vec<String>>();
    for file in filenames {
        let trimmed_filename = file.trim_end_matches(".htm");
        let dest_path = format!("site/{}.html", trimmed_filename);
        build_page(lex, file, &dest_path)?;
    }
    println!("Generated {} files", filenames.len());
    Ok(())
}

fn build_page(
    lex: &mut Lexicon,
    filename: &String,
    dest_path: &String,
) -> Result<(), std::io::Error> {
    // Check if the file exists in the inc/ directory
    if !Path::new(&format!("src/inc/{}", filename)).exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Missing file {filename}"),
        ))
    } else {
        let source_path = format!("src/inc/{}", filename);
        let mut file = File::create(dest_path)?;
        // Begin
        writeln!(file, "<!DOCTYPE html><html lang='en'>")?;
        writeln!(file, "<head>");
        write!(
            file,
            "{}",
            &format!(
                "<meta charset='utf-8'>\n
            <meta name='viewport' content='width=device-width,initial-scale=1'>\n
            <link rel='stylesheet' type='text/css' href='../links/main.css'>\n
            <title>{NAME} &mdash; {filename}</title>\n"
            )
        );
        write!(file, "</head>\n<body>\n");

        // Header
        writeln!(file, "<header>");
        write!(file,"{}",&format!("<a href='https://andrescn.me/mente/site/about.html'><img src='../media/interface/logo.svg' alt='{NAME}' height='100'></a>"));
        writeln!(file, "</header>");

        // Navigation
        writeln!(file, "<nav>");
        add_portal(&mut file, lex, "meta.nav", false)?;
        writeln!(file, "</nav>");

        // Main
        write!(file, "<main>\n\n");
        write!(file, "<!-- Generated file, do not edit -->\n\n");
        write!(file, "{}", &format!("<h1>{filename}</h1>\n"));
        add_main_body(&mut file, lex, &source_path);
        write!(file, "\n\n</main>\n");

        // Footer
        writeln!(file, "<footer id='end_footer'><hr />");
        fpedited(&mut file);
        writeln!(file, "<b>Mente</b> © 2023 — ")?;
        writeln!(file, "</footer>");
        writeln!(file, "</body></html>");

        file.flush();
        Ok(())
    }
}

fn fpedited(file: &mut File) -> Result<(), std::io::Error> {
    // Add the edited date to the footer as "YYYY-MM-DD"
    write!(file, "<span style='float:right'>")?;
    write!(file, "Edited on {}", Local::now().format("%Y-%m-%d"))?;
    write!(file, "</span>")?;
    Ok(())
}

fn generate_html_from_markdown(path: &str) -> Result<(), std::io::Error> {
    use std::process::Command;

    let entries = fs::read_dir(Path::new(path))?;

    for entry in entries {
        let entry = entry?;
        let filename = entry.file_name().into_string().unwrap();
        if filename.ends_with(".md") {
            let input_path = format!("{}/{}", path, filename);
            let output_path = format!("src/inc/{}.htm", filename.trim_end_matches(".md"));

            let status = Command::new("pandoc")
                .arg("-f commonmark")
                .arg(&input_path)
                .arg("--filter=pandoc-sidenote")
                .arg("-t")
                .arg("html5")
                .arg("-o")
                .arg(&output_path)
                .status()?;

            if !status.success() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed to run pandoc with pandoc-sidenote",
                ));
            }
        }
    }
    Ok(())
}

fn main() {
    // Before we start, we empty the ..site/ directory
    let _ = fs::remove_dir_all("site");
    let _ = fs::create_dir("site");

    // We now have also markdown files, we need to conver them
    // to .htm files and place them in the src/inc/ directory
    generate_html_from_markdown("src/md");

    let mut lexicon = Lexicon::new();
    let input_dir = Path::new("src/inc");

    // Based on the current path, we need to update input_dir
    // if we are running from the root directory we can use the default
    // otherwise we need to use the relative path

    match lexicon.index(input_dir) {
        Ok(_) => println!("Indexing Complete"),
        Err(e) => println!("Error Indexing: {}", e),
    }
    create_index_page(&lexicon);

    // Generate the HTML pages
    match generate_pages(&mut lexicon) {
        Ok(_) => println!("\nGeneration Complete"),
        Err(e) => println!("Error Generating: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read};

    #[test]
    fn test_fpinject() {
        let mut lexicon = Lexicon::new();
        lexicon.files.insert("test".to_string(), 0);

        let source_path = "src/inc/test.htm";

        // Create test file
        let _ = fs::write(source_path, "This is a {test} file");

        let mut output = File::create("src/inc/output.htm").unwrap();

        add_main_body(&mut output, &mut lexicon, source_path).unwrap();

        // Check the result
        let result = fs::read_to_string("src/inc/output.htm").unwrap();
        assert_eq!(
            result,
            "This is a <a href='test.html' class='local'>test</a> file"
        );

        // Check the references in the lexicon
        assert_eq!(lexicon.files.get("test").unwrap(), &1);

        // Clean up
        fs::remove_file(source_path).unwrap();
        fs::remove_file("src/inc/output.htm").unwrap();
    }

    #[test]
    fn test_generate_html_from_markdown() {
        // The test expects a `test` directory under the project root containing a markdown file `test.md`.
        generate_html_from_markdown("src/md").unwrap();

        let mut file = fs::File::open("src/inc/test.htm").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        assert!(contents.contains("<h1>Test</h1>"));
        fs::remove_file("src/inc/test.htm").unwrap();
    }
}
