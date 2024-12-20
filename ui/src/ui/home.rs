use crate::{
    parser::Reader,
    ui::{header::Field, state::AppState, viewer::Viewer},
};
use dioxus::prelude::*;

use super::state::Format;
#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}
        Body {}
    }
}

pub fn Header() -> Element {
    let mut current_db = use_context::<AppState>().current_db;
    let mut viewer = use_context::<AppState>().viewer;
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
                                let new_viewer = Viewer::new_from_included(name).expect("Viewer failed");
                                let first_part = new_viewer.first_part();
                                *selected_part.write() = first_part;
                                *selected_field.write() = None;
                                *viewer.write() = new_viewer;
                            }
                        };
                    },
                    // 设置不同的数据库选项
                    for name in viewer.read().included_dbnames() {
                        option {
                            selected: if *name == current_db() {"true"},
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
    let viewer = use_context::<AppState>().viewer;
    let parts = viewer.read().parts.clone();
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
            class: "flex flex-wrap p-4 text-xs",
            for field in fields {
                div {
                    div {
                        class: "mb-0 mt-1 leading-tight tracking-tighter font-medium text-green-700",
                        "{field.offset}"
                    }
                    div {
                        class: "p-1 outline outline-1 outline-secondary bg-primary hover:bg-secondary border-t-2 border-green-700",
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
