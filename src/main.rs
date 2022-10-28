mod lib;

use clap::{Parser, Subcommand};
use lib::files::FilesRaces;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    List,
    Save,
}

fn main() {
    let args: Args = Args::parse();

    let user = std::env::var_os("USER").expect("Error on get username");
    let config_path = String::from(format!(
        "/home/{}/.config",
        user.to_str().expect("Error on get username")
    ));
    let files_races = FilesRaces::new(config_path);

    match args.action {
        Action::List => {
            let actual_content = files_races.get_actual_race_content();
            for content in actual_content {
                println!("{}", content.path)
            }
        }
        Action::Save => {
            let name = String::from("cat-dotfiles");
            let actual_content = files_races.get_actual_race_content();
            let actual_content = actual_content
                .into_iter()
                .filter(|c| c.is_dir == true && !c.path.to_string().contains("dotman"))
                .collect();

            files_races.save_actual_race(name, actual_content);
        }
    }
}
