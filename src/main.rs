use regex::Regex;
use rust_translate::supported_languages::get_languages;
use rust_translate::translate;
use std::{env, fs, io, process};
#[tokio::main]
async fn main() {
    //get the inputs
    let args: Vec<String> = env::args().collect();
    let (file_path, text_lang, native_lang, langs) = parse_config(&args);
    let num_langs = langs.len() as i32;

    // respond to the inputs and flags
    println!("loading file {file_path}");

    //parse the text file into an array of lines
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let original = contents.replace('\n', "");
    let re = Regex::new(r"([.!?])").unwrap();
    let parsed = re.replace_all(&original, "$1\n");

    // loop through the lines according to user commands
    let lines: Vec<&str> = parsed.lines().collect();
    let mut counter: i32 = 0;
    loop {
        //getting current line and language
        let line_index = (counter / num_langs) as usize;
        let lang_index = (counter % num_langs) as usize;
        let current_line = lines[line_index].trim();
        let current_lang = &langs[lang_index];

        //print the translated text
        if lang_index == 0 {
            println!(
                "----------------------------{}----------------------------\n",
                line_index
            );
            println!("{}", current_line);
        } else {
            let translated_text = translate(current_line, text_lang, current_lang)
                .await
                .unwrap();
            println!("{}", translated_text);
        }

        //process user input and obey commands
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let inp = input.trim();
            if inp.is_empty() {
                counter += 1;
                break;
            } else if is_numeric(inp) {
                counter = inp.parse::<i32>().expect("could not read input") * num_langs;
                break;
            } else if lang_index as i32 == num_langs - 1 {
                println!("{}", translate(inp, native_lang, text_lang).await.unwrap());
            } else {
                println!(
                    "{}",
                    translate(inp, current_lang, native_lang).await.unwrap()
                );
            }
        }
    }
    // Translate text from any language to any other language
    // List the supported languages of the crate
}

//function to parse the input
fn parse_config(args: &[String]) -> (&str, &str, &str, &[String]) {
    if args.iter().any(|arg| arg == "-h") {
        print_help();
        process::exit(0);
    }
    let file_path = &args[1];
    (file_path, &args[2], &args[args.len() - 1], &args[2..])
}

//function that gets input in between sections
fn print_help() {
    println!("usage: ./multi_reader [file path] [languages]");
    println!("the first input language is the language of the text");
    println!("the last input language is the native language of the user");
    println!("ex: multi_reader stella_maris_espanol.txt es en zh");

    println!();
    println!("enter: next language/sentence");
    println!("num: go to num sentence");
    println!("other: translate input to native langauge");

    println!();
    let supported_languages = get_languages();
    println!("Supported languages: {:?}", supported_languages);
}
fn is_numeric(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}
