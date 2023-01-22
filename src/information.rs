//! Functions for supplying user with information 

use crate::{Config, Language, ProjectType, get_languages};

// constants 
const FLAG_WIDTH: usize = 15;
const COMMAND_WIDTH: usize = 10;

/// Basic info for how to use tool 
pub fn info() {
    println!(
        "{}\n{}", // "father string" to improve readability in code 
        // Information strings 
        "Usage: pjcr [COMMAND] [ARGUMENTS] [OPTIONS]",
        "Try 'pjcr --help' for more information."
    )
}

/// Helper function for formatting flags for help function 
fn format_flag(single_character: &str, long: &str, description: &str) -> String {
    // format string 
    format!(
        "{}{:FLAG_WIDTH$}{}",
        " ".repeat(2), // get indent to start 
        format!("{}, {}", single_character, long),
        description
    )
}

/// Helper function for formatting commands for help function 
fn format_command(command: &str, description: &str) -> String {
    // format string 
    format!(
        "{}{:COMMAND_WIDTH$}{}",
        " ".repeat(4), // get indent to start 
        command,
        description
    )
}

/// Helper function for formatting project types in help function 
fn format_project_type(project_type: ProjectType) -> String {
    format!(
        "{}{:FLAG_WIDTH$}{}",
        " ".repeat(2), // get indent to start 
        project_type.name,
        project_type.description
    )
}

/// Helper function for formatting languages in help function 
fn format_language(language: Language) -> String {
    let mut strings:  Vec<String> = vec![
        format!("{} project types: ", language.name) // start with language name
    ]; // vector of strings that make up language info 

    for project_type in language.project_types {
        strings.push(format_project_type(project_type)); // format each project type 
    }

    strings.join("\n")

}

/// Help function for how to use the tool (generally, not a specific command)
pub fn help() {
    // absolute basics 
    println!(
        "{}\n{}\n{}\n",
        "Usage: pjcr [COMMAND] [ARGUMENTS] [OPTIONS]",
        "Simple tool for creating boilerplate file structures and files for projects.",
        "Try: pjcr new python 'My Cool Project' default .", 
    );

    // Commands 
    println!("Commands:");
    println!("{}", format_command("new", "Create a new project."));


    // Flags 
    println!("\nOptional flags:");
    println!("{}", format_flag("-h", "--help", "Print help information."))


}

/// Help function for new command
pub fn new_help(cfg: Config) {
    let languages_string: Vec<String> = get_languages(&cfg); // get language strings 

    // basics 
    println!(
        "{}\n{}\n{}\n\n{}\n{}\n",
        "Usage: pjcr new [LANGUAGE] [PROJECT_NAME] [PROJECT_TYPE] [PATH]",
        "Create a new project.",
        "Example: pjcr new python 'My Cool Project' default .", 
        "Supported languages:",
        languages_string.join(", ") // join together all supported languages 
    );
    
    // print all project types 
    for language in cfg.languages {
        println!("{}", format_language(language));
    }

    // Flags 
    println!("\nOptional flags:");
    println!("{}", format_flag("-h", "--help", "Print help information."))
}
