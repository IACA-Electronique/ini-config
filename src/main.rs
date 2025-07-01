mod clap_config;
mod config_file;
mod file_reader;
mod file_writer;
mod preset_manager;

use clap::Parser;
use clap_config::{Cli, Commands, PresetCommands};
use std::process;

use config_file::ConfigFile;
use file_reader::FileReader;
use file_writer::FileWriter;
use ini_config::backup_manager::BackupManager;
use ini_config::low_level::filesystem_manager::DefaultFileSystemManager;
use preset_manager::PresetManager;

const BACKUP_DIR_NAME: &str = ".dir";

struct App {
    config_file: ConfigFile,
    cli: Cli,
    backup_manager: BackupManager,
}

impl App {
    pub fn new() -> Self {
        let cli = Cli::parse();
        let config_file_path = cli.file.to_string();
        Self {
            cli,
            config_file: ConfigFile::new(),
            backup_manager: BackupManager::new(
                &config_file_path,
                &format!("{config_file_path}{BACKUP_DIR_NAME}"),
                Box::new(DefaultFileSystemManager),
            ),
        }
    }

    pub fn run(&mut self) {
        self.load_config_or_exit();
        self.handle_cli();
    }

    fn handle_cli(&mut self) {
        let path = &self.cli.file;
        match &self.cli.command {
            Commands::Set {
                section,
                param,
                value,
            } => match self.config_file.set(&section, &param, &value) {
                Ok(_) => {
                    log(&format!(
                        "Successfully set {param} = {value} in section [{section}]"
                    ));
                    self.save_config();
                }
                Err(e) => error(&format!("Error setting value: {e}")),
            },
            Commands::Add {
                section,
                param,
                value,
            } => match self.config_file.add(&section, &param, &value) {
                Ok(_) => {
                    log(&format!(
                        "Successfully add {param} = {value} in section [{section}]"
                    ));
                    self.save_config();
                }
                Err(e) => error(&format!("Error setting value: {e}")),
            },
            Commands::Del { section, param } => match self.config_file.delete(&section, &param) {
                Ok(_) => {
                    log(&format!(
                        "Successfully deleted {param} from section [{section}]"
                    ));
                    self.save_config()
                }
                Err(e) => error(&format!("Error deleting value: {e}")),
            },
            Commands::Preset { directory, action } => match action {
                PresetCommands::Load { preset_name } => {
                    let preset_manager = PresetManager::new(directory, &path);
                    match self.backup_manager.create() {
                        Ok(_) => {
                            log("Backup created.\n");
                            match preset_manager.load(&preset_name) {
                                Ok(_) => {
                                    log(&format!("Successfully loaded preset: {preset_name}"));
                                }
                                Err(e) => {
                                    error(&format!("Error loading preset '{preset_name}' : {e}"))
                                }
                            }
                        }
                        Err(e) => {
                            error(&format!("Unable to create backup: {e}"));
                        }
                    }
                }
            },
            Commands::ListBackup { .. } => match self.backup_manager.list() {
                Ok(backups) => {
                    if backups.is_empty() {
                        log("No backups found.\n");
                    } else {
                        let mut i: usize = 0;
                        for backup in backups {
                            log(&format!("{i} | {backup}"));
                            i = i + 1;
                        }
                    }
                }
                Err(e) => error(&format!("Error listing backups: {e}")),
            },
            Commands::Restore { index } => match self.backup_manager.restore(index.clone()) {
                Ok(_) => {
                    log("Backup restored.\n");
                }
                Err(e) => error(&format!("Error restoring backup: {e}")),
            },
        }
    }

    fn save_config(&mut self) {
        let path = &self.cli.file;
        let writer = FileWriter::new(path);
        match self.backup_manager.create() {
            Ok(_) => log("Backup created.\n"),
            Err(e) => {
                error(&format!("Unable to create backup: {e}"));
                process::exit(11);
            }
        }
        match writer.write(&self.config_file.to_string()) {
            Ok(_) => log("Config file saved.\n"),
            Err(e) => {
                error(&format!("Unable to write config file to '{path}': {e}"));
                process::exit(10);
            }
        }
    }

    fn load_config_or_exit(&mut self) {
        let path = &self.cli.file;
        let reader = FileReader::new(path);

        match reader.read() {
            Ok(content) => {
                self.config_file.load(&content);
                log("✅ Config file loaded.\n\n")
            }
            Err(e) => {
                error(&format!("Unable to read config file: {e}"));
                process::exit(1);
            }
        }
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}

fn log(message: &str) {
    println!("{message}");
}

fn error(message: &str) {
    eprintln!("{message}");
}
