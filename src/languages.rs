use std::collections::HashMap;
use color_please::{set_fg, Color};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Language {
  C,
  Cpp,
  Python,
  JavaScript,
  Java,
  Rust,
  Go,
  Kotlin,
  PHP,
  HTML,
  CSS,
  SASS,
  SCSS,
  Vue,
  Vim,
  Svelte,
  JSX,
  LESS,
  Arduino,
  CSharp,
  R,
  Ruby,
  SQL,
  Assembly,
  Groovy,
  Swift,
  TypeScript,
  Perl,
  Batch,
  Shell,
  PowerShell,
  Dart,
  ObjectiveC,
  Scala,
  Haskell,
  Clojure,
  Elixir,
  Lisp,
  Lua,
  Erlang,
  Julia,
  F,
  FSharp,
  ASP,
  Crystal,
  Brainfuck,
  Proprietary,
  Pony,
  V,
  Solidity,
  Markdown,
}

pub fn extensions<'a>() -> HashMap<&'a str,Language> {
   vec![
  ("c", Language::C),
  ("cpp", Language::Cpp),
  ("c++", Language::Cpp),
  ("h", Language::C),
  ("hpp", Language::Cpp),
  ("h++", Language::Cpp),
  ("py", Language::Python),
  ("pyw", Language::Python),
  ("pyc", Language::Proprietary),
  ("js", Language::JavaScript),
  ("cjs", Language::JavaScript),
  ("mjs", Language::JavaScript),
  ("java", Language::Java),
  ("class", Language::Proprietary),
  ("jar", Language::Proprietary),
  ("rs", Language::Rust),
  ("rlib", Language::Proprietary),
  ("exe", Language::Proprietary),
  ("dll", Language::Proprietary),
  ("go", Language::Go),
  ("kt", Language::Kotlin),
  ("kts", Language::Kotlin),
  ("php", Language::PHP),
  ("html", Language::HTML),
  ("htm", Language::HTML),
  ("css", Language::CSS),
  ("sass", Language::SASS),
  ("scss", Language::SCSS),
  ("less", Language::LESS),
  ("ino", Language::Arduino),
  ("cs", Language::CSharp),
  ("r", Language::R),
  ("rb", Language::Ruby),
  ("rbw", Language::Ruby),
  ("jsx", Language::JSX),
  ("sql", Language::SQL),
  ("s", Language::Assembly),
  ("asm", Language::Assembly),
  ("groovy", Language::Groovy),
  ("gvy", Language::Groovy),
  ("gy", Language::Groovy),
  ("gsh", Language::Groovy),
  ("swift", Language::Swift),
  ("ts", Language::TypeScript),
  ("dts", Language::TypeScript),
  ("pl", Language::Perl),
  ("bat", Language::Batch),
  ("sh", Language::Shell),
  ("ps1", Language::PowerShell),
  ("dart", Language::Dart),
  ("m", Language::ObjectiveC),
  ("mm", Language::ObjectiveC),
  ("scala", Language::Scala),
  ("sc", Language::Scala),
  ("hs", Language::Haskell),
  ("lhs", Language::Haskell),
  ("clj", Language::Clojure),
  ("cljs", Language::Clojure),
  ("cljc", Language::Clojure),
  ("ex", Language::Elixir),
  ("exs", Language::Elixir),
  ("lsp", Language::Lisp),
  ("lisp", Language::Lisp),
  ("lua", Language::Lua),
  ("erl", Language::Erlang),
  ("hrl", Language::Erlang),
  ("beam", Language::Proprietary), // Erlang VM
  ("jl", Language::Julia),
  ("f", Language::F),
  ("fs", Language::FSharp),
  ("asp", Language::ASP),
  ("aspx", Language::ASP),
  ("cr", Language::Crystal),
  ("bf", Language::Brainfuck),
  ("vue", Language::Vue),
  ("vim", Language::Vim),
  ("pony", Language::Pony),
  ("svelte", Language::Svelte),
  ("so", Language::Proprietary),
  ("o", Language::Proprietary),
  ("md", Language::Markdown),
  ("sol", Language::Solidity),
  ("v", Language::V),
  ("", Language::Proprietary),
].into_iter().collect()
}

impl std::fmt::Display for Language {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      &Language::C => write!(f,"C"),
      &Language::Cpp => write!(f,"C++"),
      &Language::Python => write!(f,"Python"),
      &Language::JavaScript => write!(f,"JavaScript"),
      &Language::Java => write!(f,"Java"),
      &Language::Rust => write!(f,"Rust"),
      &Language::Go => write!(f,"Go"),
      &Language::Kotlin => write!(f,"Kotlin"),
      &Language::PHP => write!(f,"PHP"),
      &Language::HTML => write!(f,"HTML"),
      &Language::CSS => write!(f,"CSS"),
      &Language::SASS => write!(f,"SASS"),
      &Language::SCSS => write!(f,"SCSS"),
      &Language::Vue => write!(f,"Vue"),
      &Language::Vim => write!(f,"Vim"),
      &Language::Svelte => write!(f,"Svelte"),
      &Language::JSX => write!(f,"JSX"),
      &Language::LESS => write!(f,"LESS"),
      &Language::Arduino => write!(f,"Arduino"),
      &Language::CSharp => write!(f,"C#"),
      &Language::R => write!(f,"R"),
      &Language::Ruby => write!(f,"Ruby"),
      &Language::SQL => write!(f,"SQL"),
      &Language::Assembly => write!(f,"Assembly"),
      &Language::Groovy => write!(f,"Groovy"),
      &Language::Swift => write!(f,"LESS"),
      &Language::TypeScript => write!(f,"TypeScript"),
      &Language::Perl => write!(f,"Perl"),
      &Language::Batch => write!(f,"Batchfile"),
      &Language::Shell => write!(f,"Shell"),
      &Language::PowerShell => write!(f,"PowerShell"),
      &Language::Dart => write!(f,"Dart"),
      &Language::ObjectiveC => write!(f,"Objective-C"),
      &Language::Scala => write!(f,"Scala"),
      &Language::Haskell => write!(f,"Haskell"),
      &Language::Clojure => write!(f,"Clojure"),
      &Language::Elixir => write!(f,"Elixir"),
      &Language::Lisp => write!(f,"Lisp"),
      &Language::Lua => write!(f,"Lua"),
      &Language::Erlang => write!(f,"Erlang"),
      &Language::Julia => write!(f,"Julia"),
      &Language::F => write!(f,"F"),
      &Language::FSharp => write!(f,"F#"),
      &Language::ASP => write!(f,"ASP"),
      &Language::Crystal => write!(f,"Crystal"),
      &Language::Brainfuck => write!(f,"Brainfuck"),
      &Language::V => write!(f,"V"),
      &Language::Solidity => write!(f,"Solidity"),
      &Language::Markdown => write!(f,"Markdown"),
      &Language::Pony => write!(f,"Pony"),
      &Language::Proprietary => write!(f,"Potentially Proprietary Malware"),
    }
  }
}

impl Language {
  pub fn set_color_for(&self){
    match self {
      &Language::C => set_fg(Color::BrightCyan),
      &Language::Cpp => set_fg(Color::BrightCyan),
      &Language::Python => set_fg(Color::Color256(75)),
      &Language::JavaScript => set_fg(Color::BrightYellow),
      &Language::Java => set_fg(Color::Color256(130)),
      &Language::Rust => set_fg(Color::Color256(202)),
      &Language::Go => set_fg(Color::BrightCyan),
      &Language::Kotlin => set_fg(Color::Color256(92)),
      &Language::PHP=> set_fg(Color::Color256(98)),
      &Language::HTML => set_fg(Color::Color256(202)),
      &Language::CSS => set_fg(Color::BrightBlue),
      &Language::SASS => set_fg(Color::Color256(206)),
      &Language::SCSS => set_fg(Color::Color256(206)),
      &Language::Vue => set_fg(Color::Color256(78)),
      &Language::Vim => set_fg(Color::BrightGreen),
      &Language::Svelte => set_fg(Color::Color256(196)),
      &Language::JSX => set_fg(Color::BrightCyan),
      &Language::LESS => set_fg(Color::White),
      &Language::Arduino => set_fg(Color::Color256(73)),
      &Language::CSharp => set_fg(Color::Color256(92)),
      &Language::R => set_fg(Color::BrightBlue),
      &Language::Ruby => set_fg(Color::Red),
      &Language::SQL => set_fg(Color::Yellow),
      &Language::Assembly => set_fg(Color::BrightGreen),
      &Language::Groovy => set_fg(Color::Color256(38)),
      &Language::Swift => set_fg(Color::Color256(208)),
      &Language::TypeScript => set_fg(Color::Color256(39)),
      &Language::Perl => set_fg(Color::Color256(39)),
      &Language::Batch => set_fg(Color::Color256(250)),
      &Language::Shell => set_fg(Color::BrightGreen),
      &Language::PowerShell => set_fg(Color::Color256(39)),
      &Language::Dart => set_fg(Color::Color256(44)),
      &Language::ObjectiveC => set_fg(Color::White),
      &Language::Scala => set_fg(Color::Red),
      &Language::Haskell => set_fg(Color::Color256(56)),
      &Language::Clojure => set_fg(Color::Green),
      &Language::Elixir => set_fg(Color::Color256(129)),
      &Language::Lisp => set_fg(Color::White),
      &Language::Lua => set_fg(Color::Blue),
      &Language::Erlang => set_fg(Color::Color256(124)),
      &Language::Julia => set_fg(Color::White),
      &Language::F => set_fg(Color::Cyan),
      &Language::FSharp => set_fg(Color::Cyan),
      &Language::ASP => set_fg(Color::BrightBlue),
      &Language::Crystal => set_fg(Color::White),
      &Language::Brainfuck => set_fg(Color::White),
      &Language::Proprietary => set_fg(Color::Red),
      &Language::Solidity => set_fg(Color::Color256(94)),
      &Language::Markdown => set_fg(Color::Color256(246)),
      &Language::Pony => set_fg(Color::White),
      &Language::V => set_fg(Color::Color256(74)),
    };
  }
}