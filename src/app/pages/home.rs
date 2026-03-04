use leptos::prelude::*;
use crate::app::components::engine_card::EngineStatusCard;
use crate::app::hooks::use_engine::use_engine;
use crate::app::components::{
    title_card::TitleCard,
    stat_card::StatCard,
    log_viewer::LiveLogViewer,
    divider::Divider,
};

#[component]
pub fn HomePage() -> impl IntoView {
    let engine = use_engine();

    let current_data = Signal::derive(move || {
        engine.status.get().and_then(|res| res.ok())
    });

    let is_running_sig = Signal::derive(move || current_data.get().map(|d| d.is_api_running).unwrap_or(false));
    let logs_sig = Signal::derive(move || current_data.get().map(|d| d.logs).unwrap_or_default());

    view! {
        <div class="page-container">
            <Transition fallback=|| view!{ <div>"Connecting to Database..."</div> }>
                
                <Show 
                    when=move || current_data.get().is_some()
                    fallback=|| view! { 
                        <div class="card error">"Connection Error!"</div> 
                    }
                >
                    <div class="stats-grid">
                        <TitleCard
                            title="System Overview" 
                            subtitle="Lyrics Statistics"
                            icon="⚡"
                        />
                        
                        <EngineStatusCard is_running=is_running_sig />
                        
                        <StatCard 
                            label="Total amount of songs in database" 
                            value=move || current_data.get().map(|d| d.songs_amount.to_string()).unwrap_or_default()
                        />
                        <StatCard 
                            label="Songs with synced lyrics"
                            value=move || current_data.get().map(|d| d.songs_synced.to_string()).unwrap_or_default()
                        />
                        <StatCard 
                            label="Songs with plain lyrics only"
                            value=move || current_data.get().map(|d| d.songs_plain.to_string()).unwrap_or_default()
                        />
                        <StatCard 
                            label="Songs with no lyrics available"
                            value=move || current_data.get().map(|d| d.songs_noresult.to_string()).unwrap_or_default()
                        />
                        <StatCard 
                            label="Badly tagged songs"
                            value=move || current_data.get().map(|d| d.songs_tagerr.to_string()).unwrap_or_default()
                        />
                        <StatCard 
                            label="New, not yet analyzed songs"
                            value=move || current_data.get().map(|d| d.songs_unaccounted.to_string()).unwrap_or_default()
                        />
                        <StatCard 
                            label="Songs locked by user"
                            value=move || current_data.get().map(|d| d.songs_locked.to_string()).unwrap_or_default()
                        />
                    </div>
                    
                    <Divider text="Console Logs"/>
                    
                    <LiveLogViewer logs=logs_sig />

                </Show>
            </Transition>
        </div>
    }
}
