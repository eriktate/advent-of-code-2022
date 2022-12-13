#[derive(Debug)]
enum Token<'a> {
    Prompt,
    Cd,
    Ls,
    Dir,
    Root,
    PrevDir,
    StringLit(&'a str),
    IntLit(u32),
}

impl<'a> Token<'a> {
    fn parse(word: &'a str) -> Token<'a> {
        match word {
            "$" => Token::Prompt,
            "cd" => Token::Cd,
            "ls" => Token::Ls,
            "dir" => Token::Dir,
            "/" => Token::Root,
            ".." => Token::PrevDir,
            _ => match word.parse::<u32>() {
                Ok(lit) => Token::IntLit(lit),
                Err(_) => Token::StringLit(word),
            },
        }
    }
}

#[derive(Debug)]
enum Directory<'a> {
    Root,
    Prev,
    Path(&'a str),
}

#[derive(Debug)]
enum Contents<'a> {
    Dir(&'a str),
    File((u32, &'a str)),
}

#[derive(Debug)]
enum Command<'a> {
    Cd(Directory<'a>),
    Ls(Vec<Contents<'a>>),
}

#[derive(Debug)]
enum ParsingContext {
    Dir,
    File,
    Unknown,
}

impl<'a> Command<'a> {
    fn parse_cd(tokens: &mut impl Iterator<Item = Token<'a>>) -> Command<'a> {
        let tok = tokens.next().expect("expected path for cd");
        match tok {
            Token::Root => Command::Cd(Directory::Root),
            Token::PrevDir => Command::Cd(Directory::Prev),
            Token::StringLit(lit) => Command::Cd(Directory::Path(lit)),
            _ => panic!("unexpected token {:?} while parsing cd", tok),
        }
    }

    fn parse_ls(tokens: &mut impl Iterator<Item = Token<'a>>) -> Command<'a> {
        let mut contents: Vec<Contents<'a>> = Vec::new();
        let mut ctx = ParsingContext::Unknown;
        let mut size: u32 = 0;

        for tok in tokens {
            match ctx {
                ParsingContext::Unknown => match tok {
                    Token::Dir => ctx = ParsingContext::Dir,
                    Token::IntLit(lit) => {
                        ctx = ParsingContext::File;
                        size = lit;
                    }
                    Token::Prompt => return Command::Ls(contents),
                    _ => panic!("unexpected token {:?} while parsing ls", tok),
                },
                ParsingContext::Dir => match tok {
                    Token::StringLit(lit) => {
                        contents.push(Contents::Dir(lit));
                        ctx = ParsingContext::Unknown;
                    }
                    _ => panic!("unexpected token {:?} while parsing directory output", tok),
                },
                ParsingContext::File => match tok {
                    Token::StringLit(lit) => {
                        contents.push(Contents::File((size, lit)));
                        ctx = ParsingContext::Unknown;
                    }
                    _ => panic!("unexpected token {:?} while parsing file output", tok),
                },
            }
        }

        Command::Ls(contents)
    }
}

fn parse<'a>(tokens: Vec<Token<'a>>) -> Vec<Command<'a>> {
    let mut commands: Vec<Command<'a>> = Vec::new();
    let mut ctx: ParsingContext = ParsingContext::Unknown;

    let mut tok_iter = tokens.into_iter();
    while let Some(tok) = tok_iter.next() {
        match ctx {
            ParsingContext::Unknown => match tok {
                Token::Prompt => continue,
                Token::Cd => {
                    commands.push(Command::parse_cd(&mut tok_iter));
                    ctx = ParsingContext::Unknown;
                }
                Token::Ls => {
                    commands.push(Command::parse_ls(&mut tok_iter));
                    ctx = ParsingContext::Unknown;
                }
                _ => panic!("unexpected token {:?}", tok),
            },
            _ => panic!("unexpected parsing context {:?} at top level", ctx),
        }
    }

    commands
}

fn lex<'a>(input: &'a str) -> Vec<Token<'a>> {
    input
        .split_whitespace()
        .map(|word| Token::parse(word))
        .collect()
}

fn part_one<'a>(input: &'a str) {
    let tokens = lex(input);
    let commands = parse(tokens);
    for cmd in commands {
        println!("{:?}", cmd);
    }
}

fn main() {
    let input = include_str!("../example.txt");
    part_one(input);
}
