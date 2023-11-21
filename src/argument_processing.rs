use std::fs;
use std::io;

use crate::information::{help, info, new_help};
use crate::{get_languages, Config, Language, ProjectType};

/// Error object
pub enum Error {
    MissingArguments(String), // User is missing arguments.
    InvalidLanguage(String),  // User supplied an invalid language.
}

/// The raw argument parser: takes in arguments from command line and desides what to do with them
pub fn parse_arguments(args: Vec<String>, cfg: Config) -> Result<(), Error> {
    println!("Arguments: {:?}\n", args); // print arguments for debugging

    // If user supplied 0 arguments give user basic info (the first argument is just file)
    if args.len() == 1 {
        info(); // give basic info
        return Ok(()); // end program here
    }

    // we now know there is AT LEAST 1 arg
    match args[1].as_str() {
        "-h" | "--help" | "help" => help(),       // give help info
        "new" => new_parse_arguments(args, cfg)?, // parse arguments for new command
        _ => println!("Unrecognized command: {}", args[1]), // unrecognized command
    }

    Ok(())
}

/// The new command argument parser
pub fn new_parse_arguments(args: Vec<String>, cfg: Config) -> Result<(), Error> {
    // if user supplies help flag, give only help info
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        new_help(cfg); // give help info
        return Ok(()); // end program here
    }

    // user needs to supply a language argument
    if args.len() < 3 {
        return Err(Error::MissingArguments(format!(
            "{}\n\nUsage: {}",
            "ERROR: You need to supply a language argument.",
            "pjcr new [LANGUAGE] [PROJECT NAME] [PROJECT TYPE] [PATH]",
        )));
    }

    // user needs to supply a project name argument
    if args.len() < 4 {
        return Err(Error::MissingArguments(format!(
            "{}\n\nUsage: {}",
            "ERROR: You need to supply a project name argument.",
            "pjcr new [LANGUAGE] [PROJECT NAME] [PROJECT TYPE] [PATH]",
        )));
    }

    // we now know there are AT LEAST 2 args, 2nd arg is language
    let language_strings = get_languages(&cfg); // get vector of language strings

    // check if language is valid
    if language_strings.contains(&args[2]) {
        let mut project_type: Option<String> = None; // default to no project type (will be default)
        let mut path: Option<String> = None; // default to no specific path (current directory)

        // check for project type being specified
        if args.len() > 4 {
            project_type = Some(args[4].clone());
        }

        // check for specific path
        if args.len() > 5 {
            path = Some(args[5].clone());
        }

        // check if user supplied wakatime flag
        let wakatime =
            args.contains(&String::from("--wakatime")) || args.contains(&String::from("-w"));

        create_project(
            &cfg,
            args[2].clone(),
            args[3].clone(),
            project_type,
            path,
            wakatime,
        );
    } else {
        return Err(Error::InvalidLanguage(String::from(&args[2])));
    }

    Ok(())
}

/// Function for containing logic to create projects
pub fn create_project(
    cfg: &Config,
    language: String,
    name: String,
    project_type: Option<String>,
    path: Option<String>,
    wakatime: bool,
) {
    // if no path is supplied by user, assume current path
    let base_path = match path {
        Some(path) => path,
        None => String::from("."),
    };

    // split name at spaces in order to combine with underscores
    let directory_name: Vec<&str> = name.split(' ').collect();

    let project_path = format!("{}/{}", base_path, directory_name.join("_")); // get path for project

    fs::create_dir_all(&project_path).unwrap(); // create project directory

    // find language object in config for current language. Can unwrap because we know language is safe at this point
    let language_object: Language = cfg
        .languages
        .iter()
        .find(|lang| lang.name == language)
        .unwrap()
        .clone();

    // Get project type object
    let project_type_object: ProjectType = match project_type {
        // insure that the user specified a projcet_type
        Some(project_type) => {
            match language_object // find project type user specified in language project types
                .project_types
                .iter()
                .find(|proj_type| proj_type.name == project_type)
            {
                Some(project_type) => project_type.clone(),
                None => language_object.project_types[0].clone(), // default to first project type if not found
            }
        }
        None => language_object.project_types[0].clone(), // if no project type is specified, pick first project type by default
    };

    for file in project_type_object.files {
        fs::create_dir_all(format!("{}/{}", project_path, file.path)).unwrap(); // create the necessary directory for this file

        let response = reqwest::blocking::get(&file.url)
            .unwrap() // TODO: Proper error handling here
            .text() // turn response into plain text for file
            .unwrap();

        // create file using file name and path in config
        let mut out =
            fs::File::create(format!("{}/{}/{}", &project_path, file.path, file.name)).unwrap();

        io::copy(&mut response.as_bytes(), &mut out).unwrap(); // copy data into file
    }

    // create .gitignore file
    fs::File::create(format!("{}/.gitignore", &project_path)).unwrap(); // create file to write to

    let mut gitignore_text = project_type_object.gitignore.files.join("\n"); // concat all files with new line character

    // add wakatime-project file to gitignore
    if wakatime {
        gitignore_text = gitignore_text + ".wakatime-project\n";
    }

    // write all relevant text to file
    fs::write(format!("{}/.gitignore", &project_path), gitignore_text).unwrap();

    // create wakatime project file if necessary
    if wakatime {
        fs::File::create(format!("{}/.wakatime-project", &project_path)).unwrap(); // create file

        fs::write(format!("{}/.wakatime-project", &project_path), &name).unwrap();
        // write name of project to wakatime file
    }
}
