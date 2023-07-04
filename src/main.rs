use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
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
    fs::write("inc/index.htm", index)
}

fn generate(lex: &Lexicon) -> Result<(), std::io::Error> {
    for (filename, _) in &lex.files {
        let trimmed_filename = filename.trim_end_matches(".htm");
        let dest_path = format!("../site/{}.html", trimmed_filename);
        build_page(&lex, &filename, &dest_path)?;
    }
    println!("Generated {} pages", lex.files.len());
    Ok(())
}

fn build_page(lex: &Lexicon, filename: &String, dest_path: &String) -> Result<(), std::io::Error> {
    // Check if the file exists
    if !Path::new(&filename).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        ));
    } else {
        let mut f = File::create(dest_path)?;
        let mut html = String::new();
        html.push_str("<!DOCTYPE html><html lang='en'>\n");
        html.push_str("<head>\n");
        html.push_str(&format!(
            "<meta charset='utf-8'>\n
            <meta name='thumbnail' content='{DOMAIN}/media/services/thumbnail.jpg' />\n
            <meta name='viewport' content='width=device-width,initial-scale=1'>\n
            <link rel='stylesheet' type='text/css' href='../links/main.css'>\n
            <title>{NAME} &mdash; {filename}</title>\n",
        ));
        html.push_str("</head>\n<body>\n");
        html.push_str("<header>\n");
        html.push_str(&format!("<a href='home.html'><img src='../media/interface/logo.svg' alt='{NAME}' height='50'></a>\n"));
        html.push_str("</header>\n");
        html.push_str("<nav>\n");
        // TODO: Implement the fpportal equivalent function here and replace the placeholder below.
        // If the fpportal function fails, we might print an error message and continue.
        html.push_str("</nav>\n");
        html.push_str("<main>\n\n");
        html.push_str("<!-- Generated file, do not edit -->\n\n");
        html.push_str(&format!("<h1>{filename}</h1>\n"));
        // TODO: Implement the fpinject equivalent function here and replace the placeholder below.
        // If the fpinject function fails, we might print an error message and continue.
        html.push_str("\n\n</main>\n");
        html.push_str("<footer><hr />\n");
        // TODO: Implement the fpedited equivalent function here and replace the placeholder below.
        html.push_str(&format!("<b>Hundredrabbits</b> © 2022 — \n"));
        html.push_str("</footer>\n");
        html.push_str("</body></html>\n");
        fs::write(dest_path, html).expect("Unable to write file");
        Ok(())
    }
}

fn main() {
    let mut lexicon = Lexicon::new();
    let input_dir = Path::new("inc");

    match lexicon.index(input_dir) {
        Ok(_) => println!("Indexing Complete"),
        Err(e) => println!("Error Indexing: {}", e),
    }
    create_index_page(&lexicon);

    // Generate the HTML pages

    // Check for orphan pages (pages that are not linked)
}
