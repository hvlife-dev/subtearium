use leptos::prelude::*;
use std::collections::HashMap;
use crate::server::state::SongStatus;
use crate::app::components::offset_modal::OffsetModal;
use crate::app::hooks::use_engine::EngineController;

#[component]
pub fn LibraryExplorer(
    #[prop(into)] library: HashMap<String, SongStatus>,
    engine: EngineController,
) -> impl IntoView {
    let (filter_status, set_filter) = signal(None::<SongStatus>);
    let (search_query, set_search) = signal("".to_string());
    let (selected_item, set_selected) = signal(None::<(String, SongStatus)>);

    let lib_for_filter = library.clone();
    let lib_for_count = library;

    let filtered_items = move || {
        let q = search_query.get().to_lowercase();
        let s = filter_status.get();
        
        let mut items: Vec<_> = lib_for_filter.iter().collect();
        items.sort_by_key(|(k, _)| *k);

        items
            .into_iter()
            .filter(|(path, status)| {
                let status_match = match &s {
                    Some(target) => *status == target,
                    None => true,
                };
                let search_match = if q.is_empty() {
                    true
                } else {
                    path.to_lowercase().contains(&q)
                };
                status_match && search_match
            })
            .map(|(k, v)| (k.clone(), v.clone())) 
            .collect::<Vec<(String, SongStatus)>>()
    };

    let get_count = move |target: Option<SongStatus>| {
        lib_for_count.values().filter(|s| match &target {
            Some(t) => *s == t,
            None => true
        }).count()
    };

    view! {
        <div class="card" style="display: flex; flex-direction: column; height: clamp(400px, 80vh, 1200px); padding: 0; position: relative;">
            
            <div class="filter-header" style="padding: 1rem; border-bottom: 1px solid #212234; background: #212234;">
                <div style="display: flex; gap: 0.5rem; margin-bottom: 1rem; overflow-x: auto;">
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
                    placeholder="Search paths..."
                    on:input=move |ev| set_search.set(event_target_value(&ev))
                    prop:value=search_query
                />
            </div>

            <div style="flex: 1; overflow-y: auto; padding: 0;">
                <ul style="list-style: none; margin: 0; padding: 0;">
                    <For
                        each=filtered_items
                        key=|(path, _)| path.to_string()
                        children=move |(path, status)| {
                            let path_text = path.clone(); 
                            let path_title = path.clone();
                            let click_path = path.clone();
                            let click_status = status.clone();

                            view! {
                                <li 
                                    class="library-item" 
                                    style="cursor: pointer;"
                                    on:click=move |_| {
                                        set_selected.set(Some((click_path.clone(), click_status.clone())));
                                    }
                                >
                                    <StatusIcon status={status.clone()} />
                                    <span class="path" title=path_title>
                                        {path_text}
                                    </span>
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
