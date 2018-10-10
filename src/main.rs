mod config;
mod constants;
mod fs_helpers;
mod utils;
mod templ;

use config::Config;
use std::env;
use constants::{COMPONENT_NAME, STYLE_NAME, FILE_NAME};
use fs_helpers::{create_file, create_folder};
use utils::{replace_template, replace_underline_to_dash, transform_to_camel_case};
use templ::{COMPONENT_TEMPLATE, STYLE_TEMPLATE, INDEX_TEMPLATE};

fn main() {
    let args = env::args();
    let conf = Config::prepare(args);
    let component_name = transform_to_camel_case(&conf.name);
    let style_name = replace_underline_to_dash("sm-", &conf.name);
    let mut component_content = COMPONENT_TEMPLATE.to_string();
    let mut style_content = STYLE_TEMPLATE.to_string();
    let mut index_content = INDEX_TEMPLATE.to_string();

    // create dir
    create_folder(&conf.path);

    // create component file
    replace_template(&mut component_content, COMPONENT_NAME, &component_name);
    replace_template(&mut component_content, FILE_NAME, &conf.name);
    replace_template(&mut component_content, STYLE_NAME, &style_name);
    create_file(&conf.path, &conf.name, "jsx", &component_content);

    // create style file
    replace_template(&mut style_content, STYLE_NAME, &style_name);
    create_file(&conf.path, &conf.name, "scss", &style_content);

    // create index file
    replace_template(&mut index_content, FILE_NAME, &conf.name);
    create_file(&conf.path, "index", "js", &index_content);
}
