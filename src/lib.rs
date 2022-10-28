pub mod files {
    use std::{fs, io, path::Path};

    #[derive(Debug)]
    pub struct Content {
        pub path: String,
        pub is_dir: bool,
    }

    impl Content {
        fn new(path: String, is_dir: bool) -> Content {
            Self { path, is_dir }
        }
    }

    #[derive(Debug)]
    pub struct Race {
        pub name: String,
        pub path: String,
        pub content: Vec<Content>,
    }

    impl Race {
        pub fn new(path: String, content: Vec<Content>) -> Race {
            let dirs = path
                .as_str()
                .split("/")
                .map(|e| String::from(e))
                .collect::<Vec<String>>();
            let legth = dirs.len() - 1;
            let name = &dirs[legth];
            Self {
                name: name.to_owned(),
                path,
                content,
            }
        }
    }

    pub struct FilesRaces {
        pub config_path: String,
    }

    impl FilesRaces {
        pub fn new(config_path: String) -> FilesRaces {
            Self { config_path }
        }

        pub fn get_config_path() -> String {
            let user = std::env::var_os("USER").expect("Error on get username");
            String::from(format!(
                "/home/{}/.config",
                user.to_str().expect("Error on get username")
            ))
        }

        pub fn get_races(&self) -> Vec<Race> {
            let path = format!("{}/dotman/races", self.config_path);
            let entries = fs::read_dir(path).expect("Error getting entires");
            let mut path_entries = vec![];
            for entry in entries {
                let entry = entry.expect("Error getting entry");
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    if path.is_dir() {
                        let entries = fs::read_dir(&path_str).expect("Error getting entires");
                        let mut content = vec![];
                        for entry in entries {
                            let entry = entry.expect("Error getting entry");
                            let path = entry.path();
                            if let Some(path_str) = path.to_str() {
                                content.push(Content::new(path_str.to_string(), path.is_dir()));
                            }
                        }

                        path_entries.push(Race::new(path_str.to_string(), content));
                    }
                }
            }

            path_entries
        }

        pub fn get_actual_race_content(&self) -> Vec<Content> {
            let entries =
                fs::read_dir(self.config_path.to_string()).expect("Error getting entires");
            let mut contents = vec![];
            for entry in entries {
                let entry = entry.expect("Error getting entry");
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    contents.push(Content::new(path_str.to_string(), path.is_dir()));
                }
            }

            contents
        }

        pub fn save_actual_race(&self, name: String, contents: Vec<Content>) {
            for content in contents {
                let dirs = content.path.as_str().split("/").collect::<Vec<&str>>();
                let prefix = dirs[dirs.len() - 1];
                let content_path = format!("{}/dotman/races/{}/{}", self.config_path, name, prefix);
                let to_path = Path::new(content_path.as_str());
                match self.copy_recursively(&content.path, to_path) {
                    Ok(_) => {}
                    Err(error) => {
                        println!("Error during saving '{}' because '{}'", content.path, error);
                    }
                }
            }
        }

        pub fn copy_recursively(
            &self,
            source: impl AsRef<Path>,
            destination: impl AsRef<Path>,
        ) -> io::Result<()> {
            fs::create_dir_all(&destination)?;
            for entry in fs::read_dir(source)? {
                let entry = entry?;
                let filetype = entry.file_type()?;
                if filetype.is_dir() {
                    self.copy_recursively(
                        entry.path(),
                        destination.as_ref().join(entry.file_name()),
                    )?;
                } else {
                    fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
                }
            }
            Ok(())
        }

        pub fn load_race(&self, name: String) {
            let config_path = FilesRaces::get_config_path();
            let race_path = String::from_iter([config_path, String::from(format!("/{}", name))]);

            
        }
    }
}
