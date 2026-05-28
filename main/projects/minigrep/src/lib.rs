use colored::Colorize;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
    pub no_color: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path or directory"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            path,
            ignore_case,
            no_color: false,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.path);
    if path.is_dir() {
        search_dir(path, &config.query, config.ignore_case, config.no_color)?;
    } else {
        search_file(path, &config.query, config.ignore_case, config.no_color)?;
    }
    Ok(())
}

fn search_dir(
    dir: &Path,
    query: &str,
    ignore_case: bool,
    no_color: bool,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            search_dir(&path, query, ignore_case, no_color)?;
        } else {
            search_file(&path, query, ignore_case, no_color)?;
        }
    }
    Ok(())
}

fn search_file(
    path: &Path,
    query: &str,
    ignore_case: bool,
    no_color: bool,
) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Ok(()), // skip binary or unreadable files
    };

    let matches = search(query, &contents, ignore_case);
    if !matches.is_empty() {
        let file_label = path.display().to_string();
        if no_color {
            println!("--- {file_label} ---");
        } else {
            println!("{}", format!("--- {file_label} ---").cyan().bold());
        }
        for line in matches {
            println!("{}", highlight(line, query, ignore_case, no_color));
        }
    }
    Ok(())
}

// 高亮匹配关键词
fn highlight(line: &str, query: &str, ignore_case: bool, no_color: bool) -> String {
    if no_color {
        return line.to_string();
    }
    let compare_line = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };
    let compare_query = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    let mut result = String::new();
    let mut remaining = line;
    let mut remaining_lower = compare_line.as_str();

    while let Some(pos) = remaining_lower.find(&compare_query) {
        result.push_str(&remaining[..pos]);
        result.push_str(&remaining[pos..pos + query.len()].red().bold().to_string());
        remaining = &remaining[pos + query.len()..];
        remaining_lower = &remaining_lower[pos + query.len()..];
    }
    result.push_str(remaining);
    result
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    if ignore_case {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    } else {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
