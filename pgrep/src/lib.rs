use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    path::PathBuf,
    sync::Arc,
    thread,
};

pub fn find_string_in_file(search_str: &str, file: &PathBuf) -> Result<Vec<String>, Error> {
    let f = File::open(file)?;
    let mut result = vec![];
    let buf_reader = BufReader::new(f);
    for line in buf_reader.lines() {
        let str = line?;
        if str.contains(search_str) {
            result.push(str);
        }
    }
    Ok(result)
}

pub fn find_string_in_multiple_files<'a>(
    search_str: &str,
    files: &'a Vec<PathBuf>,
) -> HashMap<&'a PathBuf, Result<Vec<String>, Error>> {
    let mut result: HashMap<&PathBuf, Result<Vec<String>, Error>> = HashMap::new();
    for file in files {
        match find_string_in_file(search_str, file) {
            Ok(lines) => result.insert(file, Ok(lines)),
            Err(e) => result.insert(file, Err(e)),
        };
    }
    result
}

pub fn find_string_in_multiple_files_async_native_threads_with_data_clone<'a>(
    search_str: &str,
    files: &'a Vec<PathBuf>,
) -> HashMap<&'a PathBuf, Result<Vec<String>, Error>> {
    let mut handles = vec![];
    for file in files {
        let file_clone = file.clone();
        let search_string_clone = String::from(search_str);
        handles.push((
            file,
            thread::spawn(move || {
                let file_clone_local = file_clone;
                let search_string_local = search_string_clone;
                find_string_in_file(&search_string_local, &file_clone_local)
            }),
        ));
    }
    handles
        .into_iter()
        .map(|t| (t.0, t.1.join().unwrap()))
        .collect()
}

pub fn find_string_in_multiple_files_async_native_threads_with_arc<'a>(
    search_str: &str,
    files: &'a Vec<PathBuf>,
) -> HashMap<&'a PathBuf, Result<Vec<String>, Error>> {
    let mut handles = vec![];
    for file in files {
        let file_clone = Arc::new(file.clone());
        let search_string_clone = Arc::new(String::from(search_str));
        handles.push((
            file,
            thread::spawn(move || {
                find_string_in_file(&search_string_clone.clone(), &file_clone.clone())
            }),
        ));
    }
    handles
        .into_iter()
        .map(|t| (t.0, t.1.join().unwrap()))
        .collect()
}

pub fn find_string_in_multiple_files_async_native_threads_with_thread_scope<'a>(
    search_str: &str,
    files: &'a Vec<PathBuf>,
) -> HashMap<&'a PathBuf, Result<Vec<String>, Error>> {
    thread::scope(|s| {
        let mut handles = vec![];
        for file in files {
            handles.push(s.spawn(move || (file, find_string_in_file(search_str, file))));
        }
        handles.into_iter().map(|t| t.join().unwrap()).collect()
    })
}

#[cfg(test)]
mod tests {

    type MultipleFilesSearchFunctionType =
        for<'a> fn(
            search_str: &str,
            files: &'a Vec<PathBuf>,
        ) -> HashMap<&'a PathBuf, Result<Vec<String>, Error>>;

    use std::io::ErrorKind;

    use super::*;

    const MULTIPLE_FILES_SEARCH_FN: MultipleFilesSearchFunctionType = find_string_in_multiple_files;

    fn check_length_and_items_single_file(
        desired_size: usize,
        desired_value: Vec<&str>,
        actual_value: Vec<String>,
    ) {
        assert_eq!(actual_value.len(), desired_size);
        if desired_value.len() != actual_value.len() {
            panic!("Desired size and actual size are not the same");
        }
        for i in 0..desired_size {
            assert_eq!(actual_value.get(i).unwrap(), desired_value.get(i).unwrap())
        }
    }

    #[test]
    fn test_single_file_find_zero_lines() {
        let search_str = "ABRACADABRA";
        let file_path = std::path::PathBuf::from("tests/data/simple_logs.logs");
        let found_lines = find_string_in_file(search_str, &file_path);
        assert!(found_lines.is_ok());
        let lines = found_lines.unwrap();
        assert_eq!(lines.len(), 0);
    }

    #[test]
    fn test_single_file_find_single_line() {
        let search_str = "ERROR";
        let file_path = std::path::PathBuf::from("tests/data/simple_logs.logs");
        let found_lines = find_string_in_file(search_str, &file_path);
        assert!(found_lines.is_ok());
        let lines = found_lines.unwrap();
        check_length_and_items_single_file(
            1,
            vec!["2026-09-11T10:01:05Z ERROR connection refused"],
            lines,
        );
    }

    #[test]
    fn test_single_file_find_multiple_lines() {
        let search_str = "INFO";
        let file_path = std::path::PathBuf::from("tests/data/simple_logs.logs");
        let found_lines = find_string_in_file(search_str, &file_path);
        assert!(found_lines.is_ok());
        let lines = found_lines.unwrap();
        check_length_and_items_single_file(
            2,
            vec![
                "2026-09-11T10:01:03Z INFO server started",
                "2026-09-11T10:01:03Z INFO server has started",
            ],
            lines,
        );
    }

    #[test]
    fn test_multiple_files_two_files_exists() {
        let search_str = "INFO";
        let file_paths = vec![
            std::path::PathBuf::from("tests/data/simple_logs.logs"),
            std::path::PathBuf::from("tests/data/simple_logs_2.logs"),
        ];
        let file_name_result_map = MULTIPLE_FILES_SEARCH_FN(search_str, &file_paths);
        assert_eq!(file_name_result_map.len(), 2);
        for (file, lines_result) in file_name_result_map {
            assert!(lines_result.is_ok());
            let lines = lines_result.unwrap();
            match file.to_str().unwrap() {
                "tests/data/simple_logs.logs" => check_length_and_items_single_file(
                    2,
                    vec![
                        "2026-09-11T10:01:03Z INFO server started",
                        "2026-09-11T10:01:03Z INFO server has started",
                    ],
                    lines,
                ),
                "tests/data/simple_logs_2.logs" => {
                    check_length_and_items_single_file(1, vec!["INFO try to connect"], lines)
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_multiple_files_one_file_exists_and_one_doesnt() {
        let search_str = "INFO";
        let file_paths = vec![
            std::path::PathBuf::from("tests/data/simple_logs.logs"),
            std::path::PathBuf::from("non-existing/abracadabra/file"),
        ];
        let file_name_result_map = MULTIPLE_FILES_SEARCH_FN(search_str, &file_paths);
        assert_eq!(file_name_result_map.len(), 2);
        for (file, lines_result) in file_name_result_map {
            match file.to_str().unwrap() {
                "tests/data/simple_logs.logs" => {
                    assert!(lines_result.is_ok());
                    let lines = lines_result.unwrap();
                    check_length_and_items_single_file(
                        2,
                        vec![
                            "2026-09-11T10:01:03Z INFO server started",
                            "2026-09-11T10:01:03Z INFO server has started",
                        ],
                        lines,
                    );
                }
                "non-existing/abracadabra/file" => {
                    assert!(lines_result.is_err());
                    let error = lines_result.unwrap_err();
                    assert_eq!(error.kind(), ErrorKind::NotFound);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_multiple_files_both_files_dont_exist() {
        let search_str = "INFO";
        let file_paths = vec![
            std::path::PathBuf::from("non-existing/abracadabra/file2"),
            std::path::PathBuf::from("non-existing/abracadabra/file"),
        ];
        let file_name_result_map = MULTIPLE_FILES_SEARCH_FN(search_str, &file_paths);
        assert_eq!(file_name_result_map.len(), 2);
        for (file, lines_result) in file_name_result_map {
            assert!(lines_result.is_err());
            let error = lines_result.unwrap_err();
            match file.to_str().unwrap() {
                "non-existing/abracadabra/file2" => {
                    assert_eq!(error.kind(), ErrorKind::NotFound);
                }
                "non-existing/abracadabra/file" => {
                    assert_eq!(error.kind(), ErrorKind::NotFound);
                }
                _ => unreachable!(),
            }
        }
    }
}
