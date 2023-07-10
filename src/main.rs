use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use chrono::{DateTime, Local};

const NAME: &str = "Mente";
const DOMAIN: &str = "https://men.te";
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

fn fptemplate(file: &mut File, lex: &mut Lexicon, filename: &str) -> io::Result<()> {
    match lex.find_file(filename) {
        Some(_) => {
            let link = format!(
                "<a href='{}.html' class='local'>{}</a>",
                filename.replace(" ", "_").to_lowercase(),
                filename.replace("_", " ")
            );
            lex.increment_ref(filename);
            write!(file, "{}", link)
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Missing link: {}", filename),
        )),
    }
}

fn fpinject(file: &mut File, lex: &mut Lexicon, source_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(source_path)?;
    let mut output = String::new();
    let mut temp = String::new();
    let mut in_template = false;

    for c in content.chars() {
        match c {
            '{' => {
                in_template = true;
            }
            '}' => {
                in_template = false;
                fptemplate(file, lex, &temp)?;
                temp.clear();
            }
            _ if in_template => {
                temp.push(c);
            }
            _ => {
                output.push(c);
            }
        }
    }
    write!(file, "{}", output)
}

fn fpportal(file: &mut File, lex: &mut Lexicon, s: &str, head: bool) -> io::Result<()> {
    let filename = s.replace(" ", "_").to_lowercase();
    let filepath = format!("src/inc/{}.htm", filename);

    match lex.find_file(&filename) {
        Some(_) => {
            let page_content = fs::read_to_string(filepath)?;
            if head {
                writeln!(
                    file,
                    "<h2 id='{}'><a href='{}.html'>{}</a></h2>",
                    filename, s, s
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
    for (filename, _) in &lex.files {
        // If the file is an index.htm file, skip it
        if filename == "index.htm" {
            continue;
        }
        index.push_str(&format!(
            "<a href=\"{}\">{}</a><br>",
            filename,
            filename.trim_end_matches(".htm")
        ));
    }
    index.push_str("</body></html>");
    fs::write("src/inc/index.htm", index)
}

fn generate(lex: &mut Lexicon) -> std::io::Result<()> {
    let filenames = &lex.files.keys().cloned().collect::<Vec<String>>();
    for file in filenames {
        let trimmed_filename = file.trim_end_matches(".htm");
        let dest_path = format!("site/{}.html", trimmed_filename);
        build_page(lex, &file, &dest_path)?;
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
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Missing file {filename}"),
        ));
    } else {
        let source_path = format!("src/inc/{}", filename).to_string();
        let mut file = File::create(dest_path)?;
        // Begin
        write!(file, "<!DOCTYPE html><html lang='en'>\n")?;
        write!(file, "<head>\n");
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
        write!(file, "<header>\n");
        write!(file,"{}",&format!("<a href='home.html'><img src='../media/interface/logo.svg' alt='{NAME}' height='50'></a>"));
        write!(file, "</header>\n");

        // Navigation
        write!(file, "<nav>\n");
        fpportal(&mut file, lex, "meta.nav", true)?;
        write!(file, "</nav>\n");

        // Main
        write!(file, "<main>\n\n");
        write!(file, "<!-- Generated file, do not edit -->\n\n");
        write!(file, "{}", &format!("<h1>{filename}</h1>\n"));
        fpinject(&mut file, lex, &source_path);
        write!(file, "\n\n</main>\n");

        // Footer
        write!(file, "<footer><hr />\n");
        fpedited(&mut file);
        write!(file, "<b>Mente</b> © 2023 — \n")?;
        write!(file, "</footer>\n");
        write!(file, "</body></html>\n");

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

fn main() {
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
    match generate(&mut lexicon) {
        Ok(_) => println!("\nGeneration Complete"),
        Err(e) => println!("Error Generating: {}", e),
    }
}
