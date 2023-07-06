use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    ops::IndexMut,
    path::Path,
};

const NAME: &str = "Mente";
const DOMAIN: &str = "https://men.te";

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

fn fpportal(f: &mut File, lex: &mut Lexicon, s: &str, head: bool) -> Result<(), std::io::Error> {
    let filename = s.replace(" ", "_").to_lowercase().to_string();
    let filepath = format!("src/inc/{}.htm", filename);
    let filepath = Path::new(&filepath);
    let target = match filename {
        "meta.nav" => lex.find_file("nav.htm"),
        _ => lex.find_file(&filename),
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
        let mut html = String::new();

        // Begin
        html.push_str("<!DOCTYPE html><html lang='en'>\n");
        html.push_str("<head>\n");
        html.push_str(&format!(
            "<meta charset='utf-8'>\n
            <meta name='viewport' content='width=device-width,initial-scale=1'>\n
            <link rel='stylesheet' type='text/css' href='../links/main.css'>\n
            <title>{NAME} &mdash; {filename}</title>\n",
        ));
        html.push_str("</head>\n<body>\n");

        // Header
        html.push_str("<header>\n");
        html.push_str(&format!("<a href='home.html'><img src='../media/interface/logo.svg' alt='{NAME}' height='50'></a>"));
        html.push_str("</header>\n");

        // Navigation
        html.push_str("<nav>\n");
        fpportal(&mut f, lex, "meta.nav", true)?;
        html.push_str("</nav>\n");

        // Main
        html.push_str("<main>\n\n");
        html.push_str("<!-- Generated file, do not edit -->\n\n");
        html.push_str(&format!("<h1>{filename}</h1>\n"));
        // TODO: Implement the fpinject equivalent function here and replace the placeholder below.
        // If the fpinject function fails, we might print an error message and continue.
        html.push_str("\n\n</main>\n");

        // Footer
        html.push_str("<footer><hr />\n");
        // TODO: Implement the fpedited equivalent function here and replace the placeholder below.
        html.push_str(&format!("<b>Mente</b> © 2023 — \n"));
        html.push_str("</footer>\n");
        html.push_str("</body></html>\n");
        fs::write(dest_path, html).expect("Unable to write file");
        Ok(())
    }
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

    // Check for orphan pages (pages that are not linked)
}
