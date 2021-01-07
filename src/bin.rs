use atom::{CWD, REPORT, PROMPT, INCOMPLETE_PROMPT, format_error, Error, Environment, Value, ProgramParser, PRELUDE_FILENAME, HISTORY_FILENAME};
use rustyline::{
    error::ReadlineError,
    Editor, Helper, Modifiers, KeyEvent, Cmd
};

use std::{borrow::Cow::{self, Borrowed, Owned}, env::current_dir, fs::read_to_string, sync::{Arc, Mutex}};

use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::config::OutputStreamType;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::{MatchingBracketValidator, Validator, ValidationContext, ValidationResult};
use rustyline::{CompletionType, Config, Context, EditMode};
use rustyline_derive::Helper;

#[derive(Helper)]
struct AtomHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
    env: Environment
}

impl AtomHelper {
    fn set_prompt(&mut self, prompt: impl ToString) {
        self.colored_prompt = prompt.to_string();
    }

    fn update_env(&mut self, env: &Environment) {
        self.env = env.clone();
    }
}

impl Completer for AtomHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        if let Ok(mut path) = self.env.get_cwd() {
            let mut segment = String::new();
    
            if !line.is_empty() {
                for (i, ch) in line.chars().enumerate() {
                    if ch.is_whitespace() || ch == ';' || ch == '\'' || ch == '(' || ch == ')' || ch == '{' || ch == '}' || ch == '"' {
                        segment = String::new();
                    } else {
                        segment.push(ch);
                    }
    
                    if i == pos {
                        break;
                    }
                }
    
                if !segment.is_empty() {
                    path.push(segment.clone());
                }
            }
    
            let path_str = (Value::Path(path).to_string() + if segment.is_empty() { "/" } else { "" }).replace("/./", "/").replace("//", "/");
            let (pos, mut pairs) = self.completer.complete(path_str.as_str(), path_str.len(), ctx)?;
            for pair in &mut pairs {
                pair.replacement = String::from(line) + &pair.replacement.replace(&path_str, "");
            }
            Ok((pos, pairs))
        } else {
            self.completer.complete(line, pos, ctx)
        }
    }
}

impl Hinter for AtomHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for AtomHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for AtomHelper {
    fn validate(
        &self,
        _: &mut ValidationContext,
    ) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }

    fn validate_while_typing(&self) -> bool {
        self.validator.validate_while_typing()
    }
}


fn readline(prompt: impl ToString, rl: &mut Editor<impl Helper>) -> String {
    loop {
        match rl.readline(&prompt.to_string()) {
            Ok(line) => {
                return line
            },
            Err(ReadlineError::Interrupted) => {
                return String::new();
            },
            Err(ReadlineError::Eof) => {
                return String::new();
            },
            Err(err) => {
                eprintln!("error: {:?}", err);
            }
        }
    }
}

fn repl(atomic_rl: Arc<Mutex<Editor<AtomHelper>>>, atomic_env: Arc<Mutex<Environment>>) -> Result<(), Error> {
    loop {
        let mut env = atomic_env.lock().unwrap();
        let mut rl = atomic_rl.lock().unwrap();

        let prompt = format!("{}", Value::Apply(Box::new(env.get(PROMPT)?), vec![Value::Path(env.get_cwd()?)]).eval(&mut env)?);

        rl.helper_mut().expect("No helper").set_prompt(format!("{}", prompt));
        rl.helper_mut().expect("No helper").update_env(&env);
        let mut text = readline(prompt, &mut rl);

        if let Ok(parsed) = ProgramParser::new().parse(&text) {
            let _ = Value::Apply(Box::new(env.get(REPORT)?), vec![match parsed.eval(&mut env) {
                Ok(val) => val,
                Err(e)  => Value::Error(Box::new(e))
            }]).eval(&mut env);
            rl.add_history_entry(text.as_str());
        } else if text.trim() != "" {
            rl.bind_sequence(KeyEvent::new('\t', Modifiers::NONE), Cmd::Insert(1, String::from("    ")));
            loop {
                let err_prompt = format!("{}", Value::Apply(Box::new(env.get(INCOMPLETE_PROMPT)?), vec![Value::Path(env.get_cwd()?)]).eval(&mut env)?);
                rl.helper_mut().expect("No helper").set_prompt(format!("{}", &err_prompt));
                rl.helper_mut().expect("No helper").update_env(&env);
                let tmp = readline(&err_prompt, &mut rl);

                match ProgramParser::new().parse(&text) {
                    Ok(parsed) => {
                        let _ = Value::Apply(Box::new(env.get(REPORT)?), vec![match parsed.eval(&mut env) {
                            Ok(val) => val,
                            Err(e)  => Value::Error(Box::new(e))
                        }]).eval(&mut env);
                        rl.add_history_entry(text.as_str());
                        break
                    }
                    Err(e) => {
                        if tmp.trim() == "" {
                            let _ = Value::Apply(Box::new(env.get(REPORT)?), vec![
                                Value::Error(Box::new(Error::SyntaxError(format!("\n{}", format_error(text.as_str(), e)))))
                            ]).eval(&mut env);
                            break
                        } else { text += &tmp }
                    }
                }
            }

            rl.unbind_sequence(KeyEvent::new('\t', Modifiers::NONE));
        }
        if rl.save_history(&env.get_home_dir()?.join(HISTORY_FILENAME)).is_err() {
            eprintln!("could not save history")
        } 
    }
}

fn main() -> Result<(), Error> {
    let mut env = Environment::new();

    let config = Config::builder()
        .history_ignore_dups(true)
        .history_ignore_space(true)
        .auto_add_history(false)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .output_stream(OutputStreamType::Stdout)
        .build();
    

    let mut rl = Editor::with_config(config);

    let h = AtomHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
        env: env.clone()
    };

    rl.set_helper(Some(h));
    if rl.load_history(&env.get_home_dir()?.join(HISTORY_FILENAME)).is_err() {
        println!("No previous history.");
    }

    if let Ok(home_dir) = env.get_home_dir() {
        if let Ok(contents) = read_to_string(home_dir.join(PRELUDE_FILENAME)) {
            match ProgramParser::new().parse(&contents) {
                Ok(parsed) => match parsed.eval(&mut env) {
                    Ok(_) => {}
                    Err(e) => eprintln!("error in {}: {}", PRELUDE_FILENAME, e)
                }
                Err(e) => eprintln!("invalid syntax in {}\n{}", PRELUDE_FILENAME, format_error(&contents, e))
            }
        } else {
            eprintln!("could not read {}", PRELUDE_FILENAME)
        }
    } else {
        eprintln!("could not read {}", PRELUDE_FILENAME)
    }

    if let Ok(path) = current_dir() {
        Value::Define(
            String::from(CWD),
            Box::new(Value::Path(path))
        ).eval(&mut env)?;
    }

    let atomic_rl = Arc::new(Mutex::new(rl));
    let atomic_env = Arc::new(Mutex::new(env));
    
    let atomic_rl_clone = atomic_rl.clone();
    let atomic_env_clone = atomic_env.clone();
    
    if ctrlc::set_handler(move || {
        let _ = repl(atomic_rl_clone.clone(), atomic_env_clone.clone());
    }).is_err() {
        eprintln!("could not establish CTRL+C handler")
    }
    
    repl(atomic_rl, atomic_env)?;
    Ok(())
}