mod command;
mod lib;

use clap::Parser;
use command::command::{Action, Args};
use lib::files::FilesRaces;

fn main() {
    let args: Args = Args::parse();

    let config_path = FilesRaces::get_config_path();
    let files_races = FilesRaces::new(config_path);

    match args.action {
        Action::List => {
            let actual_content = files_races.get_actual_race_content();
            for content in actual_content {
                println!("{}", content.path)
            }
        }
        Action::Save { name } => {
            let actual_content = files_races.get_actual_race_content();
            let actual_content = actual_content
                .into_iter()
                .filter(|c| c.is_dir == true && !c.path.to_string().contains("dotman"))
                .collect();

            files_races.save_actual_race(name, actual_content);
        }
        Action::Races => {
            let races = files_races.get_races();
            for race in races {
                println!("[*] {}", race.name)
            }
        }
        Action::Load { name } => {
            files_races.load_race(name);
        }
    }
}
