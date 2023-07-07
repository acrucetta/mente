use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
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

    fn find_file(&self, filename: &str) -> Option<usize> {
        self.files.iter().position(|f| f.0 == filename)
    }
}

fn fptemplate(f: &mut File, lex: &mut Lexicon, s: &str) -> Result<(), std::io::Error> {
    let target = lex.find_file(s);
    match target {
        Some(index) => {
            let link = format!(
                "<a href='{}.html' class='local'>{}</a>",
                s.replace(" ", "_").to_lowercase(),
                s
            );
            lex.files.entry(s.to_string()).and_modify(|e| *e += 1);
            write!(f, "{}", link)?;
        }
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Missing link: {}", s),
            ))
        }
    }
    Ok(())
}

fn fpinject(f: &mut File, lex: &mut Lexicon, filepath: &str) -> Result<(), std::io::Error> {
    let content = fs::read_to_string(filepath)?;
    let mut output = String::new();
    let mut temp = String::new();
    let mut in_template = false;

    for c in content.chars() {
        if c == '{' {
            in_template = true;
            continue;
        }
        if c == '}' {
            in_template = false;
            match fptemplate(f, lex, &temp) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            temp.clear();
            continue;
        }
        if in_template {
            temp.push(c);
        } else {
            output.push(c);
        }
    }
    write!(f, "{}", output)?;
    Ok(())
}

fn fpportal(f: &mut File, lex: &mut Lexicon, s: &str, head: bool) -> Result<(), std::io::Error> {
    let mut filename = s.replace(" ", "_").to_lowercase().to_string();
    let filepath = format!("src/inc/{}.htm", filename);
    let filepath = Path::new(&filepath);

    // If the file is meta.nav, we need to add a ".htm" suffix
    if filename == "meta.nav" {
        filename.push_str(".htm");
    }

    let target = lex.find_file(&filename);

    match target {
        None => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Missing portal {}", s),
        )),
        Some(idx) => {
            let page_content = fs::read_to_string(&filepath)?;
            if head {
                write!(
                    f,
                    "<h2 id='{}'><a href='{}.html'>{}</a></h2>\n",
                    filename, filename, s
                )?;
            }
            write!(f, "{}", page_content)?;

            // Increase the reference count for the target page
            lex.files.entry(filename).and_modify(|e| *e += 1);
            Ok(())
        }
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

fn generate(lex: &mut Lexicon) -> Result<(), std::io::Error> {
    let filenames = &lex.files.keys().cloned().collect::<Vec<String>>();
    for file in filenames {
        let trimmed_filename = file.trim_end_matches(".htm");
        let dest_path = format!("site/{}.html", trimmed_filename);
        build_page(lex, &file, &dest_path)?;
    }
    print!("Generated {} files", filenames.len());
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
        let mut f = File::create(dest_path)?;

        // Begin
        write!(f, "<!DOCTYPE html><html lang='en'>\n")?;
        write!(f, "<head>\n");
        write!(
            f,
            "{}",
            &format!(
                "<meta charset='utf-8'>\n
            <meta name='viewport' content='width=device-width,initial-scale=1'>\n
            <link rel='stylesheet' type='text/css' href='../links/main.css'>\n
            <title>{NAME} &mdash; {filename}</title>\n"
            )
        );
        write!(f, "</head>\n<body>\n");

        // Header
        write!(f, "<header>\n");
        write!(f,"{}",&format!("<a href='home.html'><img src='../media/interface/logo.svg' alt='{NAME}' height='50'></a>"));
        write!(f, "</header>\n");

        // Navigation
        write!(f, "<nav>\n");
        fpportal(&mut f, lex, "meta.nav", true)?;
        write!(f, "</nav>\n");

        // Main
        write!(f, "<main>\n\n");
        write!(f, "<!-- Generated file, do not edit -->\n\n");
        write!(f, "{}", &format!("<h1>{filename}</h1>\n"));
        fpinject(&mut f, lex, dest_path);
        write!(f, "\n\n</main>\n");

        // Footer
        write!(f, "<footer><hr />\n");
        fpedited(&mut f);
        write!(f, "<b>Mente</b> © 2023 — \n")?;
        write!(f, "</footer>\n");
        write!(f, "</body></html>\n");

        f.flush();
        Ok(())
    }
}

fn fpedited(f: &mut File) -> Result<(), std::io::Error> {
    write!(f, "<span style='float:right'>")?;
    write!(f, "Edited on {}", Local::now())?;
    write!(f, "</span>")?;
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
        Ok(_) => println!("Generation Complete"),
        Err(e) => println!("Error Generating: {}", e),
    }
}
