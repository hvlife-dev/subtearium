use leptos::prelude::*;
use crate::server::state::SongStatus;

#[component]
pub fn OffsetModal(
    #[prop(into)] path: String,
    #[prop(into)] status: SongStatus,
    #[prop(into)] on_apply: Callback<(String, f32)>,
    #[prop(into)] on_cancel: Callback<()>,
) -> impl IntoView {
    let offset_buf = RwSignal::new(0.0f32);
    let apply_path = path.clone();

    view! {
        <div class="modal-overlay" on:click=move |_| on_cancel.run(())>
            
            <div class="modal-content card" on:click=|ev| ev.stop_propagation()>
                
                <h3 style="margin-bottom: 0.5rem; color: #7199EE;">"Edit Lyric Offset"</h3>
                <div style="font-size: 0.8rem; color: #4A5057; margin-bottom: 1.5rem; word-break: break-all;">
                    {path}
                </div>

                {if status != SongStatus::Synced {
                    view! {
                        <div style="color: #EE6D85; background: rgba(238, 109, 133, 0.1); padding: 1rem; border-radius: 6px; margin-bottom: 1.5rem;">
                            <strong>"Error:"</strong> " This file does not have synced lyrics (.lrc). Offsets can only be applied to synced files."
                        </div>
                        <button class="btn-primary" style="width: 100%;" on:click=move |_| on_cancel.run(())>
                            "CLOSE"
                        </button>
                    }.into_any()
                } else {
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
                        
                        <div style="display: flex; gap: 1rem; margin-top: 2rem;">
                            <button 
                                class="btn-primary" 
                                style="background: transparent; border: 1px solid #4A5057; color: #A0A8CD; flex: 1;"
                                on:click=move |_| on_cancel.run(())
                            >
                                "CANCEL"
                            </button>
                            <button 
                                class="btn-primary" 
                                style="flex: 1;"
                                on:click=move |_| {
                                    on_apply.run((apply_path.clone(), offset_buf.get()));
                                }
                            >
                                "APPLY"
                            </button>
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
