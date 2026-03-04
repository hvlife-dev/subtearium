use leptos::prelude::*;
use std::collections::HashMap;
use crate::server::state::SongStatus;
use crate::app::components::offset_modal::OffsetModal;
use crate::app::hooks::use_engine::EngineController;

#[component]
pub fn LibraryExplorer(
    #[prop(into)] library: Signal<HashMap<String, SongStatus>>, 
    engine: EngineController,
) -> impl IntoView {
    let (filter_status, set_filter) = signal(None::<SongStatus>);
    let (search_query, set_search) = signal("".to_string());
    let (selected_item, set_selected) = signal(None::<(String, SongStatus)>);

    let filtered_items = move || {
        let q = search_query.get().to_lowercase();
        let s = filter_status.get();
        
        let current_lib = library.get(); 
        
        let mut items: Vec<_> = current_lib.into_iter().collect();
        items.sort_by_key(|(k, _)| k.clone());

        items
            .into_iter()
            .filter(|(path, status)| {
                let status_match = match &s {
                    Some(target) => *status == *target,
                    None => true,
                };
                let search_match = if q.is_empty() {
                    true
                } else {
                    path.to_lowercase().contains(&q)
                };
                status_match && search_match
            })
            .collect::<Vec<(String, SongStatus)>>()
    };

    let get_count = move |target: Option<SongStatus>| {
        library.with(|lib| {
            lib.values().filter(|s| match &target {
                Some(t) => **s == *t,
                None => true
            }).count()
        })
    };

    view! {
        <div class="card library-card">
            
            <div class="filter-header">
                <div class="filter-tabs-wrapper">
                    <FilterTab 
                        label="All" 
                        active={move || filter_status.get().is_none()} 
                        count={get_count(None)}
                        on_click=move |_| set_filter.set(None) 
                    />
                    <FilterTab 
                        label="Synced" 
                        active={move || filter_status.get() == Some(SongStatus::Synced)} 
                        count={get_count(Some(SongStatus::Synced))}
                        on_click=move |_| set_filter.set(Some(SongStatus::Synced)) 
                    />
                    <FilterTab 
                        label="Plain" 
                        active={move || filter_status.get() == Some(SongStatus::Plain)} 
                        count={get_count(Some(SongStatus::Plain))}
                        on_click=move |_| set_filter.set(Some(SongStatus::Plain)) 
                    />
                    <FilterTab 
                        label="Missing" 
                        active={move || filter_status.get() == Some(SongStatus::NoResult)} 
                        count={get_count(Some(SongStatus::NoResult))}
                        on_click=move |_| set_filter.set(Some(SongStatus::NoResult)) 
                    />
                    <FilterTab 
                       label="Bad Tag" 
                       active={move || filter_status.get() == Some(SongStatus::TagErr)} 
                       count={get_count(Some(SongStatus::TagErr))}
                       on_click=move |_| set_filter.set(Some(SongStatus::TagErr)) 
                    />
                    <FilterTab 
                       label="Unknown"
                       active={move || filter_status.get() == Some(SongStatus::Unaccounted)} 
                       count={get_count(Some(SongStatus::Unaccounted))}
                       on_click=move |_| set_filter.set(Some(SongStatus::Unaccounted)) 
                    />
                    <FilterTab 
                       label="Locked"
                       active={move || filter_status.get() == Some(SongStatus::Locked)} 
                       count={get_count(Some(SongStatus::Locked))}
                       on_click=move |_| set_filter.set(Some(SongStatus::Locked)) 
                    />
                </div>

                <input 
                    type="text" 
                    class="form-control" 
                    placeholder="Search songs..."
                    on:input=move |ev| set_search.set(event_target_value(&ev))
                    prop:value=search_query
                />
            </div>

            <div class="library-list-wrapper">
                <ul class="library-list">
                    <For
                        each=filtered_items
                        key=|(path, _)| path.to_string()
                        children=move |(path, status)| {
                            let click_path = path.clone();
                            let click_status = status.clone();

                            let p = std::path::Path::new(&path);
                            let file_name = p.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or(&path)
                                .to_string();
                            let parent_dir = p.parent()
                                .and_then(|parent| parent.to_str())
                                .unwrap_or("")
                                .to_string();
                            let parent_display = if parent_dir.is_empty() { 
                                "".to_string() 
                            } else { 
                                format!("{}/", parent_dir) 
                            };

                            view! {
                                <li 
                                    class="library-item" 
                                    on:click=move |_| {
                                        set_selected.set(Some((click_path.clone(), click_status.clone())));
                                    }
                                >
                                    <StatusIcon status={status.clone()} />
                                    <div class="library-item-content">
                                        <span class="library-item-name" title={file_name.clone()}>
                                            {file_name.clone()}
                                        </span>
                                        <span class="library-item-path" title={parent_display.clone()}>
                                            {parent_display.clone()}
                                        </span>
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </div>

            {move || selected_item.get().map(|(path, status)| {
                let engine_clone = engine.clone();
                let engine_clone_2 = engine.clone();
                view! {
                    <OffsetModal 
                        path=path
                        status=status
                        on_apply=move |(p, offset)| {
                            engine_clone.offset_lyric.run((p, offset));
                            set_selected.set(None);
                        }
                        on_toggle_lock=move |p| {
                            engine_clone_2.toggle_lock.run(p); 
                            set_selected.set(None);
                        }
                        on_cancel=move |_| set_selected.set(None)
                    />
                }
            })}
        </div>
    }
}

#[component]
fn FilterTab(
    label: &'static str,
    active: impl Fn() -> bool + 'static + Copy + std::marker::Send,
    count: usize,
    #[prop(into)] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button 
            class=move || if active() { "filter-tab active" } else { "filter-tab" }
            on:click=move |_| on_click.run(())
        >
            {label}
            <span class="badge">{count}</span>
        </button>
    }
}

#[component]
fn StatusIcon(status: SongStatus) -> impl IntoView {
    let (color_class, icon) = match status {
        SongStatus::Synced => ("text-synced", "✓"),
        SongStatus::Plain  => ("text-plain", "≡"),
        SongStatus::TagErr   => ("text-error", "✕"),
        SongStatus::NoResult  => ("text-warning", "!"),
        SongStatus::Unaccounted  => ("text-muted", "-"),
        SongStatus::Locked  => ("text-locked", "#"),
    };
    
    view! {
        <span class=format!("status-icon {}", color_class)>
            {icon}
        </span>
    }
}
