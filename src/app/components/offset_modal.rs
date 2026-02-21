use leptos::prelude::*;
use crate::server::state::SongStatus;

#[component]
pub fn OffsetModal(
    #[prop(into)] path: String,
    #[prop(into)] status: SongStatus,
    #[prop(into)] on_apply: Callback<(String, f32)>,
    #[prop(into)] on_cancel: Callback<()>,
    #[prop(into)] on_toggle_lock: Callback<String>,
) -> impl IntoView {
    let offset_buf = RwSignal::new(0.0f32);

    let is_locked = status == SongStatus::Locked;
    let is_synced = status == SongStatus::Synced;

    let path_text = path.clone();

    view! {
        <div class="modal-overlay" on:click=move |_| on_cancel.run(())>
            
            <div class="modal-content card" on:click=|ev| ev.stop_propagation()>
                
                <h3 class="modal-title">
                    {if is_locked { "Locked File" } else { "Edit Lyric" }}
                </h3>
                
                <div class="modal-path">
                    {path_text}
                </div>

                {if is_locked {
                    let p_unlock = path.clone();
                    
                    view! {
                        <div class="modal-alert warning">
                            "This file is protected. Unlock it to edit offsets or allow automated syncing."
                        </div>
                        <div class="modal-actions">
                            <button class="btn-primary btn-outline flex-1" on:click=move |_| on_cancel.run(())>
                                "CANCEL"
                            </button>
                            <button class="btn-primary flex-1" on:click=move |_| on_toggle_lock.run(p_unlock.clone())>
                                "UNLOCK FILE"
                            </button>
                        </div>
                    }.into_any()

                } else if !is_synced {
                    let p_lock = path.clone();
                    
                    view! {
                        <div class="modal-alert error">
                            "This file does not have synced lyrics (.lrc). Offsets can only be applied to synced files."
                        </div>
                        <div class="modal-actions">
                            <button class="btn-primary btn-outline flex-1" on:click=move |_| on_cancel.run(())>
                                "CLOSE"
                            </button>
                            <button class="btn-primary btn-warning-outline flex-1" on:click=move |_| on_toggle_lock.run(p_lock.clone())>
                                "LOCK FILE"
                            </button>
                        </div>
                    }.into_any()

                } else {
                    let p_lock2 = path.clone();
                    let p_apply = path.clone();

                    view! {
                        <div class="form-group">
                            <label>"Offset in Seconds (e.g., -2.5 or 1.5)"</label>
                            <input 
                                type="number" 
                                step="0.1" 
                                class="form-control"
                                prop:value=move || offset_buf.get().to_string()
                                on:input=move |ev| {
                                    let val = event_target_value(&ev).parse::<f32>().unwrap_or(0.0);
                                    offset_buf.set(val);
                                }
                            />
                        </div>
                        
                        <div class="modal-actions">
                            <button class="btn-primary btn-outline flex-1" on:click=move |_| on_cancel.run(())>
                                "CANCEL"
                            </button>
                            <button class="btn-primary btn-warning-outline flex-1" on:click=move |_| on_toggle_lock.run(p_lock2.clone())>
                                "LOCK"
                            </button>
                            <button class="btn-primary flex-2" on:click=move |_| on_apply.run((p_apply.clone(), offset_buf.get()))>
                                "APPLY"
                            </button>
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
