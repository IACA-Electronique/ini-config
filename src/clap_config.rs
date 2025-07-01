use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ini-config")]
#[command(about = "Configure ini file type.")]
pub struct Cli {
    #[arg(short, long, default_value = "/boot/firmware/config.txt")]
    pub file: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Set {
        #[arg(short, long, default_value = "all")]
        section: String,
        param: String,
        value: String,
    },
    Add {
        #[arg(short, long, default_value = "all")]
        section: String,
        param: String,
        value: String,
    },
    Del {
        #[arg(short, long, default_value = "all")]
        section: String,
        param: String,
    },
    Preset {
        #[command(subcommand)]
        action: PresetCommands,
        #[arg(short, long, default_value = "./presets")]
        directory: String
    },
    ListBackup,
    Restore{
        index : u8
    }
}

#[derive(Subcommand)]
pub enum PresetCommands {
    Load { preset_name: String }
}
