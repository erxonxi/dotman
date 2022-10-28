mod lib;

use lib::files::FilesRaces;

fn main() {
    let user = std::env::var_os("USER").expect("Error on get username");
    let config_path = String::from(format!("/home/{}/.config", user.to_str().expect("Error on get username")));
    let files_races = FilesRaces::new(config_path);
    let actual_content = files_races.get_actual_race_content();

    let name = String::from("cat-dotfiles");
    let actual_content = actual_content
        .into_iter()
        .filter(|c| c.is_dir == true && !c.path.to_string().contains("dotman"))
        .collect();

    files_races.save_actual_race(name, actual_content);
}
