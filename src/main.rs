use serde_derive::{Deserialize, Serialize};
use std::env;

mod argument_processing;
mod information;

use argument_processing::{parse_arguments, Error};

/// Configuration object
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub languages: Vec<Language>,
}

/// Object to represent a supported language
#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    pub name: String,                    // name of language
    pub project_types: Vec<ProjectType>, // supported project types for this language
}

/// Object to represent a supported project type
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectType {
    pub name: String,            // name of project type
    pub description: String,     // description of project type for help menu
    pub files: Vec<ProjectFile>, // links to get files from
}

/// Object to hold information for necessary project files
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectFile {
    pub name: String, // name of file
    pub path: String, // path to save file to
    pub url: String,  // url to get file from
}

// Default configuration object
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            languages: vec![Language {
                name: "latex".into(),
                project_types: vec![
                    ProjectType {
                        name: "default".into(),
                        description: "A barebones latex project without a .cls file.".into(),
                        files: vec![
                            ProjectFile {
                                name: "main.tex".into(),
                                path: ".".into(),
                                url: "https://raw.githubusercontent.com/TheSharkhead2/Project_Creator_2/main/templates/latex/default/main.tex".into(),
                            }
                        ],
                    },
                    ProjectType {
                        name: "hmcmath".into(),
                        description: "The Harvey Mudd College LaTeX homework template.".into(),
                        files: vec![
                            ProjectFile {
                                name: "main.tex".into(),
                                path: ".".into(),
                                url: "".into()
                            },
                            ProjectFile {
                                name: "hmcmath.cls".into(),
                                path: ".".into(),
                                url: "https://raw.githubusercontent.com/hmcmathematics/hmcpset-class/master/hmcpset.cls".into(),
                            }
                        ]
                    }
                ],
            }],
        }
    }
}

/// Helper function to get vector of language strings (as this is needed a lot for some reason)
pub fn get_languages(cfg: &Config) -> Vec<String> {
    cfg.languages
        .iter()
        .map(|language| language.name.clone())
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect(); // grab the command line arguments

    // load the configuration file
    let cfg: Config = match confy::load("pjcr", None) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("Error loading configuration file: {}", e);
            confy::store("pjcr", None, Config::default()).unwrap();
            Config::default()
        }
    };

    // vector of all supported languages
    let language_strings = get_languages(&cfg);

    // parse arguments and do stuff (very good documentation here)
    match parse_arguments(args, cfg) {
        Ok(_) => println!(""),
        Err(Error::MissingArguments(e)) => println!("{}", e),
        Err(Error::InvalidLanguage(e)) => {
            println!(
                "'{}' is an unsupported language.\nCurrently supported languages: {}",
                e,
                language_strings.join(", ")
            )
        }
    }
}
