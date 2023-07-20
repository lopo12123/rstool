pub struct DocImpl {}

impl DocImpl {
    /// 打开文档网页
    fn open_doc() -> Result<(), String> {
        match webbrowser::open("https://github.com/lopo12123/rstool") {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{}", err))
        }
    }

    /// 处理 Command::Doc 子命令
    pub fn handle() {
        println!("[Commands::Doc]");

        match DocImpl::open_doc() {
            Ok(_) => println!("Ok"),
            Err(err) => println!("Error: {err}"),
        };
    }
}