pub mod command {
	use clap::{Parser, Subcommand};

	#[derive(Parser, Debug)]
	pub struct Args {
			#[command(subcommand)]
			pub action: Action,
	}
	
	#[derive(Subcommand, Debug)]
	pub enum Action {
			List,
			Save{ name: String },
			Races
	}	
}