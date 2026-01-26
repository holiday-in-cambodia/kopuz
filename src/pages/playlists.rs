use dioxus::prelude::*;

#[component]
pub fn PlaylistsPage(playlist_store: Signal<crate::reader::PlaylistStore>) -> Element {
    let store = playlist_store.read();
    
    rsx! {
        div {
            class: "p-8",
            div { class: "flex items-center justify-between mb-8",
                h1 { class: "text-3xl font-bold text-white", "Playlists" }
            }
            
            if store.playlists.is_empty() {
                div { class: "flex flex-col items-center justify-center h-64 text-slate-500",
                    i { class: "fa-regular fa-folder-open text-4xl mb-4 opacity-50" }
                    p { "No playlists yet. Add songs from your library!" }
                }
            } else {
                div { class: "grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                    for playlist in &store.playlists {
                        div {
                            class: "bg-white/5 border border-white/5 rounded-2xl p-6 hover:bg-white/10 transition-all cursor-pointer group",
                            div { class: "mb-4 w-12 h-12 bg-indigo-500/20 rounded-full flex items-center justify-center text-indigo-400 group-hover:text-indigo-300 group-hover:bg-indigo-500/30 transition-colors",
                                i { class: "fa-solid fa-list-ul" }
                            }
                            h3 { class: "text-xl font-bold text-white mb-1", "{playlist.name}" }
                            p { class: "text-sm text-slate-400", "{playlist.tracks.len()} tracks" }
                        }
                    }
                }
            }
        }
    }
}
