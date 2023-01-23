# Project Creator 2 (pjcr)

This project aims to provide a simple command line tool that jumpstarts any project you have by creating all of the necessary template files. If you have the same template you use for every Python project, or you would just like to have a consistent template, this tool aims to make this easy for you! I made a really basic version of this tool a little while ago (you can find that [here](https://github.com/TheSharkhead2/Project_Creator)), though it had a lot of issues so I decided to start fresh.

## Installation 

Currently, in order to install this you will have to use [Cargo](https://github.com/rust-lang/cargo). If you have rust installed you should be fine. 

In order to install this project and add it to your path, simply run: 
```
  cargo install pjcr
```

This will compile the project and add it to your path!

## Usage 

The usage for this tool is as follows: 
``` 
  pjcr [COMMAND] [ARGUMENTS] [OPTIONS]
```

Though if all you want to do is create projects using the default templates, you can create a latex project using the barebones template like so: 
```
  pjcr new latex "My Project"
```

If you want to change the type of project or where the project is created, here are all the arguments for the new command: 
```
  pjcr new [LANGUAGE] [PROJECT NAME] [PROJECT TYPE] [PATH]
```

In order to get more information about a specific command or the tool itself, you can just add the ```--help``` flag: 
```
  pjcr new --help  
```


## Roadmap 
- Custom project support (adding new templates for individual users/systems)
- Local file (included in binary) support for default templates
- Support for having the project name be subbed into files. This can probably best be done through searching the text of each file for something like: "$PROJECT_NAME". This would let you do things like have templates that have classes named after the project itself. 
- Add git support (specifically a .gitignore to the latex templates)
- Wakatime support? (Though I started using an editor that doesn't support Wakatime so this is low priority and also not that hard to implement)