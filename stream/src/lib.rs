pub fn a() {
    #[allow(unused)]
    let home = Menu {
        title: "Rust Tools".to_string(),
        options: vec![
        "W.WinRa1n".to_string(),
        "G.GenSM BIOS".to_string(),
        "T.ProperTree".to_string(),
        "M.制作黑苹果安装磁盘".to_string(),
        "O.OpenCore".to_string(),
        "S.设置".to_string(),
        "I.OpCore-Simplify".to_string(),
        "U.USBToolBox".to_string(),
        "P.Pytools终端".to_string(),
        "A.关于".to_string(),
        "Q.退出".to_string(),
        ]
    };
}

use std::fmt;

struct Menu {
    title: String,
    options: Vec<String>,
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "#".repeat(52))?;
        writeln!(f, "#{:^50}#", self.title)?;
        writeln!(f, "{}", "#".repeat(52))?;
        for option in &self.options {
            writeln!(f, "{}", option)?;
        }
        Ok(())
    }
}
