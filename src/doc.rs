use std::str::FromStr;
use webbrowser::Browser;

pub struct DocImpl {}

impl DocImpl {
    /// 打开文档网页
    fn open_doc(_browser: Option<String>) -> Result<(), String> {
        // let target = match browser {
        //     Some(b) => Browser::from_str(&b.to_lowercase()).unwrap_or(Browser::Default),
        //     None => Browser::Default,
        // };
        //
        // match webbrowser::open_browser(target, "https://github.com/lopo12123/rstool") {
        //     Ok(_) => Ok(()),
        //     Err(err) => Err(format!("{}", err))
        // }

        match webbrowser::open("https://github.com/lopo12123/rstool") {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{}", err))
        }
    }

    /// 处理 Command::Doctor 子命令
    pub fn handle(_browser: Option<String>) {
        println!("[Commands::Doctor]");

        match DocImpl::open_doc(_browser) {
            Ok(_) => println!("Ok"),
            Err(err) => println!("Error: {err}"),
        };
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn tt() {
        match DocImpl::open_doc(Some("safari".to_string())) {
            Ok(_) => {
                println!("ok");
            }
            Err(err) => {
                println!("err");
            }
        }
    }
}