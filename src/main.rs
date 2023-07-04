use std::{collections::HashMap, fs, path::Path};

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
        index.push_str(&format!(
            "<a href=\"{}\">{}</a><br>",
            filename,
            filename.trim_end_matches(".htm")
        ));
    }
    index.push_str("</body></html>");
    fs::write("inc/index.htm", index)
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
