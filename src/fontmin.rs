use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Read;
use font_kit::font::Font;
use font_kit::loader::Loader;
use font_kit::source::SystemSource;
use crate::utils::Utils;

pub struct FontMinImpl {}

impl FontMinImpl {
    /// 获取字体源
    fn get_font(source_file: &str) -> Result<Font, String> {
        if !Utils::check_file(source_file) {
            return Err(format!("File does not exist, or is not a file"));
        }

        match File::open(source_file) {
            Ok(mut file) => match Font::from_file(&mut file, 0) {
                Ok(font) => Ok(font),
                Err(err_parse) => Err(format!("Fail to parse target file.\n{err_parse}"))
            },
            Err(err_open) => Err(format!("Fail to open target file.\n{err_open}")),
        }
    }

    /// 打印字体元信息
    ///
    /// - family_name
    /// - full_name
    /// - is_monospace
    /// - glyph_count
    /// - font_style
    /// - font_weight
    /// - font_stretch
    pub fn print_meta(font: &Font) {
        let family_name = font.family_name();
        let full_name = font.full_name();
        let is_monospace = font.is_monospace();
        let glyph_count = font.glyph_count();
        let properties = font.properties();
        let font_style = properties.style;
        let font_weight = properties.weight.0;
        let font_stretch = properties.stretch.0;

        println!("\nThe metadata of the target font is as follows:\n- family_name: {family_name}\n- is_monospace: {is_monospace}\n- full_name: {full_name}\n- glyph_count: {glyph_count}\n- font_style: {font_style}\n- font_weight: {font_weight}\n- font_stretch: {font_stretch}");
    }

    /// 解析字符子集 (去除所有控制字符和空格, Ok 已确保不为空)
    fn get_subset(subset_file: &str) -> Result<HashSet<char>, String> {
        if !Utils::check_file(subset_file) {
            return Err(format!("File does not exist, or is not a file"));
        }

        match File::open(subset_file) {
            Ok(mut file) => {
                let mut subset_str = String::new();

                match file.read_to_string(&mut subset_str) {
                    Ok(_) => {
                        let mut subset: HashSet<char> = subset_str.chars().filter(|c| !c.is_control()).collect();
                        subset.remove(&' ');

                        if subset.is_empty() {
                            Err(format!("Subset cannot be empty"))
                        } else {
                            Ok(subset)
                        }
                    }
                    Err(err_read) => Err(format!("Fail to read target file\n{err_read}")),
                }
            }
            Err(err_open) => Err(format!("Fail to open target file\n{err_open}")),
        }
    }

    /// 构建子集
    fn generate_subset(font: Font, subset_chars: HashSet<char>) {
        let handle = font.handle().unwrap();


    }

    /// 处理 Command::FontMin 子命令
    pub fn handle(input: String, output: Option<String>, chars: Option<String>) {
        println!("[Commands::Hash] input: '{input}', output: '{}', chars: '{}'",
                 match &output {
                     Some(v) => v,
                     None => "none",
                 },
                 match &chars {
                     Some(v) => v,
                     None => "none",
                 }
        );

        // 载入字体文件
        match FontMinImpl::get_font(&input) {
            Ok(font) => {
                // 打印元数据
                FontMinImpl::print_meta(&font);

                // 需要子集化
                if chars.is_some() {
                    match FontMinImpl::get_subset(&chars.unwrap()) {
                        Ok(subset_chars) => {
                            println!("\nThe number of remaining characters after removing control characters and spaces in the target subset is: {}", subset_chars.len());

                            // TODO 执行子集操作
                            println!("TODO 执行子集操作");
                            println!("{:#?}", subset_chars)
                        }
                        Err(err) => println!("Error: {err}"),
                    }
                }
            }
            Err(err) => println!("Error: {err}"),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use std::path::Path;
    use font_kit::font::Font;
    use font_kit::source::SystemSource;

    const FONT_EN: &str = r"D:\rstool\examples\fontmin\JetBrainsMono-Regular.ttf";
    const FONT_EN2: &str = r"C:\Users\20366\Downloads\JetBrainsMono\JetBrainsMono-Bold.ttf";
    const SUBSET_EN: &str = r"D:\rstool\examples\fontmin\subset_en.txt";

    #[test]
    fn tt() {
        // let mut file = File::open(FONT_EN2).unwrap();
        // let font = Font::from_file(&mut file, 0).unwrap();
        // println!("font.glyph_count(): {:?}", font.properties());

        FontMinImpl::handle(FONT_EN.to_string(), Some("".to_string()), Some(SUBSET_EN.to_string()));
    }
}