use leptos::prelude::*;
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

    view! {
        <div class="page-container">

            <Transition fallback=|| "Connecting to Database...">
                {move || engine.status.get().map(|res| match res {
                    Ok(data) => view! {
                        
                        <div class="stats-grid">
                            <TitleCard
                                title="System Overview" 
                                subtitle="Lyrics Statistics"
                                icon="⚡"
                            />
                            <StatCard 
                                label="Total amount of songs in database" 
                                value=data.songs_amount.to_string() 
                            />
                            <StatCard 
                                label="Songs with synced lyrics"
                                value=data.songs_synced.to_string() 
                            />
                            <StatCard 
                                label="Songs with plain lyrics only"
                                value=data.songs_plain.to_string() 
                            />
                            <StatCard 
                                label="Songs with no lyrics available"
                                value=data.songs_noresult.to_string() 
                            />
                            <StatCard 
                                label="Badly tagged songs"
                                value=data.songs_tagerr.to_string() 
                            />
                            <StatCard 
                                label="New, not analyzed yet songs"
                                value=data.songs_unaccounted.to_string() 
                            />
                            <StatCard 
                                label="Songs locked by user"
                                value=data.songs_locked.to_string() 
                            />
                        </div>
                        
                        <Divider text="Console Logs"/>
                        
                        <LiveLogViewer logs=data.logs />

                    }.into_any(),
                    
                    Err(e) => view! { 
                        <div class="card error">
                            "Connection Error: " {e.to_string()}
                        </div>
                    }.into_any()
                })}
            </Transition>
            
        </div>
    }
}
