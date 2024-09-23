use regex::Regex;
use std::{collections::HashSet, env, error::Error, ffi::OsStr, fs, path::PathBuf};
use testmate::TestDirectoryName;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_input = testmate::get_args()?;
    let cwd = env::current_dir()?;

    let search_target_test_dir = set_test_dir(parsed_input.test_dir);
    let test_file_list = get_test_file_list(&cwd, search_target_test_dir);

    let re = Regex::new(&format!(
        r"{}(?P<test_file_suffix>_test|Test|_Test|\.spec)(?:\.)?{}$",
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

fn get_test_file_list(cwd: &PathBuf, search_target_test_dir: HashSet<String>) -> Vec<PathBuf> {
    search_target_test_dir
        .into_iter()
        .flat_map(|dir_str| match fs::read_dir(dir_str) {
            Ok(read_dir) => read_dir
                .filter_map(|entry| entry.ok().map(|e| cwd.join(e.path())))
                .collect::<Vec<PathBuf>>(),
            Err(_) => Vec::new(),
        })
        .collect()
}

fn find_matching_test_file(re: &Regex, test_file: PathBuf) -> Option<PathBuf> {
    re.is_match(test_file.to_str()?).then(|| test_file)
}

fn set_test_dir(input_test_dir: TestDirectoryName) -> HashSet<String> {
    let default_search_dir = "tests".to_string();
    let mut search_target_dir = HashSet::new();
    search_target_dir.insert(default_search_dir); // TODO: default値を設定できるように
    input_test_dir.map(|v| search_target_dir.insert(v));

    search_target_dir
}
