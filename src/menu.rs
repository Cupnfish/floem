use glazier::{Counter, HotKey};

static COUNTER: Counter = Counter::new();

/// An entry in a menu.
///
/// An entry is either a [`MenuItem`], a submenu (i.e. [`Menu`]), or one of a few other
/// possibilities (such as one of the two options above, wrapped in a [`MenuLensWrap`]).
pub enum MenuEntry {
    Seperator,
    Item(MenuItem),
    SubMenu(Menu),
}

pub struct Menu {
    pub(crate) popup: bool,
    pub(crate) item: MenuItem,
    pub(crate) children: Vec<MenuEntry>,
}

impl From<Menu> for MenuEntry {
    fn from(m: Menu) -> MenuEntry {
        MenuEntry::SubMenu(m)
    }
}

impl Menu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            popup: false,
            item: MenuItem::new(title),
            children: Vec::new(),
        }
    }

    pub(crate) fn popup(mut self) -> Self {
        self.popup = true;
        self
    }

    /// Append a menu entry to this menu, returning the modified menu.
    pub fn entry(mut self, entry: impl Into<MenuEntry>) -> Self {
        self.children.push(entry.into());
        self
    }

    /// Append a separator to this menu, returning the modified menu.
    pub fn separator(self) -> Self {
        self.entry(MenuEntry::Seperator)
    }

    pub(crate) fn platform_menu(&self) -> glazier::Menu {
        let mut menu = if self.popup {
            glazier::Menu::new_for_popup()
        } else {
            glazier::Menu::new()
        };
        for entry in &self.children {
            match entry {
                MenuEntry::Seperator => {
                    menu.add_separator();
                }
                MenuEntry::Item(item) => {
                    menu.add_item(
                        item.id as u32,
                        &item.title,
                        item.key.as_ref(),
                        item.selected,
                        item.enabled,
                    );
                }
                MenuEntry::SubMenu(m) => {
                    let enabled = m.item.enabled;
                    let title = m.item.title.clone();
                    menu.add_dropdown(m.platform_menu(), &title, enabled);
                }
            }
        }
        menu
    }
}

pub struct MenuItem {
    pub(crate) id: u64,
    title: String,
    key: Option<HotKey>,
    selected: Option<bool>,
    enabled: bool,
    pub(crate) action: Option<Box<dyn Fn()>>,
}

impl From<MenuItem> for MenuEntry {
    fn from(i: MenuItem) -> MenuEntry {
        MenuEntry::Item(i)
    }
}

impl MenuItem {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: COUNTER.next(),
            title: title.into(),
            key: None,
            selected: None,
            enabled: true,
            action: None,
        }
    }

    pub fn action(mut self, action: impl Fn() + 'static) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
