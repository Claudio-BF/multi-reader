use regex::Regex;
use rust_translate::supported_languages::get_languages;
use rust_translate::translate;
use std::*;
const PROGRESS_FILE: &str = "/usr/local/share/multi-reader/progress.txt";
#[tokio::main]
async fn main() {
    //get the inputs
    let args: Vec<String> = env::args().collect();
    let (file_path, text_lang, native_lang, langs) = parse_config(&args);
    let num_langs = langs.len();
    //parse the text file into an array of lines
    println!("loading file {file_path}");
    let lines = get_lines(file_path);

    //
    let mut progress = get_progress();
    let (mut counter, index) = get_current_file_progress(&mut progress, file_path);
    counter *= num_langs;
    'reader: loop {
        //getting current line and language
        let line_index = counter / num_langs;
        let lang_index = counter % num_langs;
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
            //get input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let inp = input.trim();

            //process input
            if inp.is_empty() {
                counter += 1;
                break;
            } else if is_numeric(inp) {
                counter = inp.parse::<usize>().expect("could not read input") * num_langs;
                break;
            } else if inp == "q" {
                progress[index].1 = line_index;
                save_progress(&progress);
                break 'reader;
            } else if lang_index == num_langs - 1 {
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
    println!("q: quit the program and save your current page number");
    println!("other: translate input to native langauge");

    println!();
    let supported_languages = get_languages();
    println!("Supported languages: {:?}", supported_languages);
}
fn is_numeric(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}
fn get_lines(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let original = contents.replace('\n', "");
    let re = Regex::new(r"([.!?])").unwrap();
    let parsed = re.replace_all(&original, "$1\n");

    // loop through the lines according to user commands
    parsed.lines().map(|line| line.to_string()).collect()
}
fn parse_progress_line(line: &str) -> (String, usize) {
    let split = line.split_once(" ").unwrap();
    (
        split.0.rsplit('/').next().unwrap().to_owned(),
        split.1.parse::<usize>().expect("could not parse index"),
    )
}
fn get_progress() -> Vec<(String, usize)> {
    let contents =
        fs::read_to_string(PROGRESS_FILE).expect("could not read progress file at {PROGRESS_FILE}");
    contents
        .lines()
        .map(|line| parse_progress_line(line))
        .collect()
}
fn get_current_file_progress(
    all_progress: &mut Vec<(String, usize)>,
    current_file: &str,
) -> (usize, usize) {
    let current_file_parsed = current_file.rsplit('/').next().unwrap();
    for (index, (filename, line)) in all_progress.iter_mut().enumerate() {
        if filename == current_file_parsed {
            return (line.to_owned(), index);
        }
    }
    all_progress.push((current_file_parsed.to_owned(), all_progress.len()));
    (0, all_progress.len() - 1)
}
fn save_progress(all_progress: &[(String, usize)]) {
    println!("user quit, saving progress");
    let mut data = String::new();
    for (filename, progress) in all_progress.iter() {
        data.push_str(filename);
        data.push(' ');
        data.push_str(&progress.to_string());
        data.push('\n');
    }
    let result = fs::write(PROGRESS_FILE, data.trim());
    match result {
        Ok(_) => println!("file written successfully"),
        Err(e) => eprintln!("failed to write to file: {}", e),
    }
}
