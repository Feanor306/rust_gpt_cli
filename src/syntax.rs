use std::io::stdout;
use crossterm::{cursor, ExecutableCommand, terminal::{Clear, ClearType}};
use std::collections::HashMap;
use syntect::{
    easy::HighlightLines, parsing::{SyntaxReference, SyntaxSet}, 
    highlighting::{ThemeSet, Style}, util::as_24_bit_terminal_escaped,
};

#[derive(Debug)]
pub struct SyntaxHighlighter {
    pub prompt: String,
    pub ps: SyntaxSet,
    pub ts: ThemeSet,
    pub syntax: SyntaxReference,
}

impl SyntaxHighlighter {
    pub fn new(prompt: &String) -> Self {
        let pss: SyntaxSet = SyntaxSet::load_defaults_newlines();
        let sx: &SyntaxReference = pss.find_syntax_by_extension(
            get_extension_from_prompt(prompt).as_str()
        ).unwrap_or_else(|| pss.find_syntax_plain_text());

        Self {
            prompt: prompt.into(),
            ps: pss.clone(),
            ts: ThemeSet::load_defaults(),
            syntax: sx.clone(),
        }
    }

    pub fn style_line(&mut self, line: &String) -> String {
        let mut h = HighlightLines::new(&self.syntax, &self.ts.themes["base16-ocean.dark"]);
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &self.ps).unwrap();

        return as_24_bit_terminal_escaped(&ranges[..], true);
    }

    pub fn reprint_with_style(&mut self, line: &String) {
        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();
        stdout.execute(cursor::SavePosition).unwrap();

        // Go to previous line and clear it
        stdout.execute(cursor::MoveUp(1)).unwrap();
        stdout.execute(Clear(ClearType::CurrentLine)).unwrap();

        // Syntax highlighing then reprint
        print!("{}", self.style_line(line));

        stdout.execute(cursor::RestorePosition).unwrap();
        stdout.execute(cursor::Show).unwrap();
    }
}

pub fn get_extension_from_prompt(prompt: &String) -> String {
    let ext = HashMap::from([
        ("rust", "rs"),
        ("javascript", "js"),
        ("js", "js"),
        ("typescript", "ts"),
        ("ts", "ts"),
        ("go", "go"),
        ("ruby", "rb"),
        ("markdown", "md"),
    ]);
    
    for (k, v) in ext.into_iter() {
        if prompt.contains(k) {
            return v.into();
        }
    }
    
    return "txt".into();
}
