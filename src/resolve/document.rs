const BRIEF_INTRO: &'static str
= "A Command Line Application For Generating And Searching Notes\n\
For more infomation, use the command \"help\"\n\
Author: NeoClear/Yolf\n\
Github Repo: https://github.com/NeoClear/rote\n";

const VERSION: &'static str
= "v0.0.1";

const AUTHOR: &'static str
= "NeoClear/Yolf";

const GITHUB: &'static str
= "https://github.com/NeoClear/rote";

const HELP_INFO: &'static str
= "Version: v0.0.1\n\
Author: NeoClear/Yolf\n\
Github: https://github.com/NeoClear/rote\n\
usage: rote command -subcommand \"description\"";

pub fn brief_intro() -> &'static str {
    BRIEF_INTRO
}

pub fn help_info() -> &'static str {
    HELP_INFO
}