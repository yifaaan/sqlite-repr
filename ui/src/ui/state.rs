use std::rc::Rc;

use dioxus::signals::Signal;

use super::{viewer::Viewer, Field, Parts};

/// 全局共享状态
#[derive(Clone, Debug)]
pub struct AppState {
    /// 当前选中的数据库
    pub current_db: Signal<String>,
    pub viewer: Signal<Viewer>,
    pub selected_part: Signal<Rc<dyn Parts>>,
    pub selected_field: Signal<Option<Field>>,
    /// 字段的显示格式
    pub format: Signal<Format>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Format {
    /// 混合
    Hybrid,
    /// 16进制
    Hex,
    /// text
    Text,
}

impl AppState {
    pub fn init() -> Self {
        // 默认选中的数据库
        let start_db_name = "Simple";
        let viewer = Viewer::new_from_included(start_db_name)
            .expect("Viewer failed to init for preloaded db.");

        Self {
            current_db: Signal::new("Simple".to_string()),
            selected_part: Signal::new(viewer.first_part()),
            selected_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
            viewer: Signal::new(viewer),
        }
    }
}
