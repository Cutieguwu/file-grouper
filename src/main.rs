use std::{env, fs, io, path};

fn main() {
    // Should really use std::env for this, but I'm lazy and will recompile.
    let dir_base: &String = &String::from("/mnt/Metacrisis2");

    // Files in dir_base
    let mut files: Vec<String> = match get_files(dir_base) {
        Ok(files) => files,
        Err(err) => panic!("{err}")
    };

    // Dirs in dir_base
    let dirs: Vec<String> = match get_dirs(dir_base) {
        Ok(dirs) => dirs,
        Err(err) => panic!("{err}")
    };

    // Get all common titles and sort them longest to shortest.
    // This is to prevent issues with long titles containing short titles.
    let title_groups: Vec<String> = {
        let mut titles: Vec<String> = get_titles(files.to_owned(), ".mkv");
        
        // Sort, producing inverted pattern of a <= b
        titles.sort_by(|a, b| b.chars().count().cmp(&a.chars().count()));

        titles
    };

    for group in title_groups {
        let dir_target: &String = &format!("{}/{}", dir_base, group);

        // If target dir does not exist, make it.
        if dirs.iter().find(|dir| **dir == group).is_none() {
            match fs::create_dir(dir_target) {
                Ok(_) => (),
                Err(err) => panic!("{err}")
            }
        }

        let mut files_moved: Vec<String> = vec![];

        for file_name in &files {
            if file_name.contains(group.as_str()) {
                let from: String = format!("{}/{}", dir_base, file_name);
                let to: String = format!("{}/{}", dir_target, file_name);
                
                if let Err(err) = move_file(from.as_str(), to.as_str()) {
                    panic!("{err}")
                }

                files_moved.push(file_name.to_owned());
            }
        }

        // Forget files that have already been moved
        for file in files_moved {
            let index: usize = files.iter()
                .position(|f| *f == file)
                .unwrap();

            files.remove(index);
        }
    }

}

#[allow(dead_code)]
fn get_path() -> io::Result<String> {
    Ok(String::from(env::current_dir()?.to_str().unwrap()))
}

fn get_files(path: &str) -> io::Result<Vec<String>> {
    let entries: fs::ReadDir = fs::read_dir(path)?;

    Ok(entries.filter_map(|entry| {
            let path: path::PathBuf = entry.ok()?.path();

            if path.is_file() {
                path.file_name()?
                    .to_str()
                    .map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect())
}

fn get_dirs(path: &str) -> io::Result<Vec<String>> {
    let entries: fs::ReadDir = fs::read_dir(path)?;

    let dir_entries: Vec<String> = entries
        .filter_map(|entry| {
            let path: path::PathBuf = entry.ok()?.path();
            if path.is_dir() {
                path.file_name()?
                    .to_str()
                    .map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    Ok(dir_entries)
}

fn get_titles(
    files: Vec<String>,
    extension_pat: &str
) -> Vec<String> {
    files.iter().filter_map(|file| {
        if file.ends_with(extension_pat) {
            Some(file.strip_suffix(".mkv")?.to_string())
        } else {
            None
        }
    })
    .collect()
}

fn move_file(from: &str, to: &str) -> io::Result<()> {
    if let Err(err) = fs::rename(from, to) {
        match fs::copy(from, to) {
            Ok(_) => fs::remove_file(from),
            Err(_) => Err(err)
        }
    } else {
        Ok(())
    }
}