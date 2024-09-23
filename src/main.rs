use regex::Regex;
use std::{collections::HashSet, env, error::Error, ffi::OsStr, fs, path::PathBuf};
use testmate::TestDirectoryName;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_input = testmate::get_args()?;
    let cwd = env::current_dir()?;

    const DEFAULT_SEARCH_DIR: &str = "tests";
    let search_target_dir = match parsed_input.test_dir {
        Some(v) => v,
        None => DEFAULT_SEARCH_DIR.to_string(),
    };
    let test_file_list = get_test_file_list(&cwd, search_target_dir);

    let re = Regex::new(&format!(
        r"{}(?P<test_file_suffix>_test|Test|_Test|\.spec)(?:\.)?{}$", // TODO: パターンを指定できるように
        &parsed_input.file.file_stem().unwrap().to_string_lossy(),
        &parsed_input
            .file
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_string_lossy()
    ))
    .unwrap();

    let matching_test_file_list = test_file_list
        .into_iter()
        .flat_map(|file| find_matching_test_file(&re, file))
        .collect::<Vec<PathBuf>>();

    handle_output(matching_test_file_list);

    Ok(())
}

fn handle_output(matching_test_file_list: Vec<PathBuf>) -> () {
    match matching_test_file_list.len() {
        0 => {
            // TODO: touchフラグに応じてテストファイルを作成できるように
            eprint!("{}", "Test file not found");
            ()
        }
        1 => {
            print!(
                "{}",
                matching_test_file_list.first().unwrap().to_string_lossy()
            );
        }
        _ => {
            eprint!("{}", "Find files too much");
        }
    }
}

fn get_recursive_test_files() -> Option<PathBuf> {
    None
}

fn get_test_file_list(
    cwd: &PathBuf,
    search_target_dir: String,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    match fs::read_dir(search_target_dir) {
        Ok(read_dir) => read_dir.flat_map(|dir_entry| {
            dir_entry.map(|v| {
                v.path()
                    .is_dir()
                    .then(|| get_test_file_list(cwd, v.path().to_str().unwrap().to_string()));
                v.path().is_file().then(|| v.path());
            })
        }),
        Err(_) => Ok(Vec::<PathBuf>::new()),
    }

    // search_target_test_dir
    //     .into_iter()
    //     .flat_map(|dir_str| match fs::read_dir(dir_str) {
    //         Ok(read_dir) => read_dir
    //             .filter_map(|entry| entry.ok().map(|e| cwd.join(e.path())))
    //             .collect::<Vec<PathBuf>>(),
    //         Err(_) => Vec::new(),
    //     })
    //     .collect()
}

fn find_matching_test_file(re: &Regex, test_file: PathBuf) -> Option<PathBuf> {
    re.is_match(test_file.to_str()?).then(|| test_file)
}
