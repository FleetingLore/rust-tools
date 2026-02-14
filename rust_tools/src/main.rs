// 每一个菜单页面，包含若干选项
struct MenuNode {
    name: String,
    title: String,
    options: Vec<MenuOption>,
}

// 跳转到每一个菜单页面的选项
struct MenuOption {
    token: String,
    info: String,
    goto_index: usize,
}

// 菜单数据
struct MenuSystem {
    menus: Vec<MenuNode>,
    void_index: usize,
}

impl MenuSystem {
    fn new() -> Self {
        let mut system = MenuSystem {
            menus: Vec::new(),
            void_index: 0,
        };

        // 创建 void 菜单
        system.void_index = system.add_menu_node("void", "void");
        system
    }

    fn add_menu_node(&mut self, name: &str, title: &str) -> usize {
        let index = self.menus.len();
        self.menus.push(MenuNode {
            name: name.to_string(),
            title: title.to_string(),
            options: Vec::new(),
        });
        index
    }

    fn register_menus(&mut self) {
        let menu_data = vec![
            ("home", "Pytools"),
        ];

        for (name, title) in menu_data {
            self.add_menu_node(name, title);
        }
    }

    fn register_options(&self) -> Vec<MenuOption> {
        let option_data = vec![
            ("W", "WinRa1n"),
            ("G", "GenSM BIOS"),
            ("T", "ProperTree"),
            ("M", "制作黑苹果安装磁盘"),
            ("O", "OpenCore"),
            ("S", "设置"),
            ("I", "OpCore-Simplify"),
            ("U", "USBToolBox"),
            ("P", "Pytools终端"),
            ("A", "关于"),
            ("Q", "退出"),
        ];

        option_data.into_iter()
            .map(|(token, info)| MenuOption {
                token: token.to_string(),
                info: info.to_string(),
                goto_index: self.void_index,
            })
            .collect()
    }

    fn map_home(&mut self) {
        let home_index = 1; // home 是第二个添加的菜单 (void 是 0, home 是 1)
        let options = self.register_options();

        for option in options {
            self.menus[home_index].options.push(option);
        }
    }

    fn make_status_map(&mut self) {
        self.map_home();
    }

    fn start(&self) {
        let home_index = 1;
        display_menu(&self.menus[home_index]);
    }
}

fn display_title(title: &str) {
    println!("{}", "#".repeat(52));
    println!("#{:^50}#", title);
    println!("{}", "#".repeat(52));
}

fn display_options(options: &[MenuOption]) {
    for option in options {
        println!("{} | {}", option.token, option.info);
    }
}

fn display_menu(menu: &MenuNode) {
    display_title(&menu.title);
    display_options(&menu.options);
}

fn main() {
    let mut menu_system = MenuSystem::new();
    menu_system.register_menus();
    menu_system.make_status_map();
    menu_system.start();
}