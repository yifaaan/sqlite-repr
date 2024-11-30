#![allow(non_snake_case)]

use std::include_bytes;
use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use ui::parser::Reader;
use ui::ui::{Field, Parts};

pub const SIMPLE_DB: &'static [u8] = include_bytes!("../examples/simple");
pub const BIG_PAGE_DB: &'static [u8] = include_bytes!("../examples/big_page");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

/// 全局共享状态
#[derive(Clone, Debug)]
pub struct AppState {
    /// 数据库实例
    pub db_examples: HashMap<&'static str, &'static [u8]>,
    /// 当前选中的数据库
    pub current_db: Signal<String>,
    /// 数据库读取器
    pub current_reader: Signal<Reader>,
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
        let start_db_bytes = SIMPLE_DB;

        let reader = Reader::new(start_db_bytes).unwrap();
        let first_part = reader.parts[0].clone();
        Self {
            db_examples: HashMap::from([
                (start_db_name, start_db_bytes),
                ("Big Page", BIG_PAGE_DB),
            ]),
            current_db: Signal::new("Simple".to_string()),
            current_reader: Signal::new(reader),
            selected_part: Signal::new(first_part),
            selected_field: Signal::new(None),
            format: Signal::new(Format::Hybrid),
        }
    }
}

fn App() -> Element {
    use_context_provider(|| AppState::init());
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Header {}
        Body {}
    }
}

pub fn Header() -> Element {
    let db_examples = use_context::<AppState>().db_examples;
    let mut current_db = use_context::<AppState>().current_db;
    let mut current_reader = use_context::<AppState>().current_reader;
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;
    rsx! {
        div {
            class: "h-12 flex items-center bg-primary",

            div {
                class: "text-xl font-bold tracking-tighter pl-4",
                "SQLite File Format"
            }

            div { class: "flex-grow" }

            div {
                class: "join",

                div {
                    class: "join-item btn btn-secondary tracking-tighter font-bold",
                    "Example database"
                }

                // 下拉菜单选择对应数据库
                select {
                    class: "join-item select select-secondary select-bordered font-bold tracking-tighter",

                    oninput: move |e| {
                        match e.value().as_str() {
                            // 选择对应的数据库
                            name => {
                                *current_db.write() = name.to_string();
                                let db_bytes = db_examples.get(name).unwrap();
                                let reader = Reader::new(db_bytes).expect("Reader failed");
                                let first_part = reader.parts[0].clone();
                                *selected_part.write() = first_part;
                                *selected_field.write() = None;
                                *current_reader.write() = reader;
                            }
                        };
                    },
                    // 设置不同的数据库选项
                    for (name, f) in &db_examples {
                        option {
                            selected: *name == current_db().as_str(),
                            "{name}",
                        }
                    }
                }
            }
            div {class: "flex-grow"}
            div {
                class: "btn btn-ghost tracking-tighter font-bold",
                "Add Yours",
            }
        }
    }
}

pub fn Body() -> Element {
    rsx! {
        div {
            class: "flex w-full",

            div {
                class: "bg-secondary",
                SideBar {}
                div { class: "flex-grow" }
            }

            div {
                class: "flex flex-col w-full",
                div {
                    Description {}
                }
                div {
                    Visual {}
                }
                div {class: "flex-grow" }
            }
        }
    }
}

/// 展示解析出的数据库结构（Parts），
/// 用户可以点击以查看详细信息。
pub fn SideBar() -> Element {
    let reader = use_context::<AppState>().current_reader;
    let parts = reader.read().parts.clone();
    let mut selected_part = use_context::<AppState>().selected_part;
    let mut selected_field = use_context::<AppState>().selected_field;
    rsx! {
        div {
            class: "rounded-box p-4 h-[calc(100vh-48px)] w-fit",
            div {
                class: "font-bold truncate pb-4",
                "Structure",
            }
            ul {
                for part in parts {
                    li {
                        button {
                            class: "w-full text-left btn-sm btn-ghost btn-block font-normal truncate",
                            class: if selected_part.read().label() == part.label() {"btn-active"},
                            onclick: move |_| {
                                *selected_part.write() = part.clone();
                                *selected_field.write() = None;
                            },
                            "+ {&part.label()}",
                        }
                    }
                }
            }
        }
    }
}

/// 显示当前选中部分或字段的描述，
/// 如果有字段被选中，则还会显示该字段的偏移、大小、值等信息。
pub fn Description() -> Element {
    let selected_part = use_context::<AppState>().selected_part;
    let selected_field = use_context::<AppState>().selected_field;
    match selected_field() {
        None => {
            rsx! {
                div {
                    class: "p-5 h-72 w-full overflow-auto",
                    "{selected_part().desc()}"
                }
            }
        }
        Some(field) => {
            rsx! {
                div {
                    class: "p-5 h-72 w-full overflow-auto",
                    div {
                        "{selected_part().desc()}"
                    }
                    div {
                        class: "flex pt-6 text-sm space-x-6",
                        // 域的描述
                        div {
                            class: "w-1/2",
                            "{field.desc}"
                        }
                        // 域的详细信息
                        div {
                            class: "overflow-auto w-1/2",
                            table {
                                class: "table table-sm",
                                tbody {
                                    tr {
                                        td {
                                            "Offset"
                                        }
                                        td {
                                            "{field.offset} byte(s)"
                                        }
                                    }
                                    tr {
                                        td {
                                            "Size"
                                        }
                                        td {
                                            "{field.size} byte(s)"
                                        }
                                    }
                                    tr {
                                        td {
                                            "Value"
                                        }
                                        td {
                                            "{field.value}"
                                        }
                                    }
                                    tr {
                                        td {
                                            "Hex"
                                        }
                                        td {
                                            "{field.to_hex()}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 按字段显示当前选中pair的field内容，
/// 提供切换格式化模式的按钮。
pub fn Visual() -> Element {
    let selected_part = use_context::<AppState>().selected_part;
    let fields = selected_part().fields();
    let mut selected_field = use_context::<AppState>().selected_field;
    let mut formatting = use_context::<AppState>().format;

    rsx! {
        div {
            class: "flex items-center bg-secondary",
            div { class: "flex-grow" }
            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                class: if formatting() == Format::Hybrid {"btn-active"},
                onclick: move |_| {
                    *formatting.write() = Format::Hybrid
                },
                "Hybrid"
            }

            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                class: if formatting() == Format::Hex {"btn-active"},
                onclick: move |_| {
                    *formatting.write() = Format::Hex
                },
                "Hex"
            }

            div {
                class: "btn btn-xs btn-ghost tracking-tighter font-bold",
                class: if formatting() == Format::Text {"btn-active"},
                onclick: move |_| {
                    *formatting.write() = Format::Text
                },
                "Text"
            }
        }

        div {
            class: "flex flex-wrap px-4 pt-3 pb-4 text-xs",
            for field in fields {
                div {
                    class: "p-1 outline outline-1 outline-secondary bg-primary mt-1 hover:bg-secondary",
                    // 选中时，显示filed的Description
                    onmouseover: move |_| {
                        *selected_field.write() = Some(field.clone());
                    },
                    FormattedValue {field: field.clone()}
                }
            }
        }
    }
}

#[component]
pub fn FormattedValue(field: Field) -> Element {
    let formatting = use_context::<AppState>().format;
    match formatting() {
        Format::Hybrid => {
            rsx! {
                div {
                    class: "divide-y divide-secondary",
                    div {
                        "{field.value}"
                    }
                    div {
                        "{field.to_hex()}"
                    }
                }
            }
        }
        Format::Hex => {
            rsx! {
                div {
                    "{field.to_hex()}"
                }
            }
        }
        Format::Text => {
            rsx! {
                div {
                    "{field.value}"
                }
            }
        }
    }
}
