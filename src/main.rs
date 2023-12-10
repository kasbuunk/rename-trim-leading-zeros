use std::fs;
use std::io;
use std::path;

fn main() -> Result<(), io::Error> {
    let directory = "input";
    let output_directory = "output";
    let file_paths = find_files(directory)?;
    let renamings = derive_renamings(&file_paths, output_directory);

    renamings.iter().try_for_each(|renaming| {
        println!(
            "renaming {:?} to {:?}",
            &renaming.current_name, &renaming.new_name
        );
        fs::rename(&renaming.current_name, &renaming.new_name)
    })
}

#[derive(PartialEq, Debug)]
struct Renaming {
    current_name: path::PathBuf,
    new_name: path::PathBuf,
}

fn derive_renamings(file_paths: &[path::PathBuf], output_directory: &str) -> Vec<Renaming> {
    file_paths
        .iter()
        .map(|file_path| Renaming {
            current_name: (*file_path.clone()).to_path_buf(),
            new_name: {
                path::PathBuf::from(output_directory).join(path::PathBuf::from(trim_leading_zeros(
                    &file_path
                        .file_name()
                        .expect("file path was not a file")
                        .to_string_lossy(),
                )))
            },
        })
        .collect()
}

fn trim_leading_zeros(s: &str) -> &str {
    match s.chars().nth(0) {
        None => s,
        Some('0') => trim_leading_zeros(&s[1..]),
        _ => s,
    }
}

fn find_files(directory: &str) -> Result<Vec<path::PathBuf>, io::Error> {
    let mut files = fs::read_dir(directory)?
        .map(|result| result.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    files.sort();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_files() {
        let files = find_files("testdata").expect("read dir error");
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn test_derive_renamings() {
        let current_file_names = vec![
            path::PathBuf::from("input/0001.1.jpg"),
            path::PathBuf::from("input/0052.8.jpg"),
        ];
        let renamings = derive_renamings(&current_file_names, "output");
        let expected_renamings = vec![
            Renaming {
                current_name: current_file_names[0].clone(),
                new_name: "output/1.1.jpg".into(),
            },
            Renaming {
                current_name: current_file_names[1].clone(),
                new_name: "output/52.8.jpg".into(),
            },
        ];
        assert_eq!(renamings, expected_renamings);
    }
}
