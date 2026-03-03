use clap::Parser;

#[derive(Parser, Clone)]
#[command(disable_help_flag = true)]
pub struct Args {

    #[arg(long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    pub file: String,

    #[arg(short = 'h', long = "file-header", help = "Display the ELF file header", default_value_t = false)]
    pub ehdr: bool,

    #[arg(short = 'S', long = "section-headers", aliases = ["sections"],
      help = "Display the section headers", default_value_t = false)]
    pub shdr: bool,

    #[arg(short = 'l', long = "program-headers", aliases = ["segments"],
      help = "Display the program headers", default_value_t = false)]
    pub phdr: bool
}
