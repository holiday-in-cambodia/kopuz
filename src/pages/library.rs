use crate::reader::Library;
use dioxus::prelude::*;

use crate::components::stat_card::StatCard;
use crate::components::playlist_modal::PlaylistModal;
use crate::components::track_row::TrackRow;
use crate::hooks::use_library_items::{use_library_items, SortOrder};

#[component]
pub fn LibraryPage(
    library: Signal<Library>,
    playlist_store: Signal<crate::reader::PlaylistStore>,
    on_rescan: EventHandler
) -> Element {
    let lib = library.read();
    
    let items = use_library_items(library);
    let mut sort_order = items.sort_order;

    let mut active_menu_track = use_signal(|| None::<std::path::PathBuf>);
    let mut show_playlist_modal = use_signal(|| false);
    let mut selected_track_for_playlist = use_signal(|| None::<std::path::PathBuf>);

    rsx! {
        div {
            class: "p-8 relative min-h-full",
            if *show_playlist_modal.read() {
                PlaylistModal {
                    playlist_store: playlist_store,
                    on_close: move |_| show_playlist_modal.set(false),
                    on_add_to_playlist: move |playlist_id: String| {
                        if let Some(path) = selected_track_for_playlist.read().clone() {
                            let mut store = playlist_store.write();
                            if let Some(playlist) = store.playlists.iter_mut().find(|p| p.id == playlist_id) {
                                if !playlist.tracks.contains(&path) {
                                    playlist.tracks.push(path);
                                }
                            }
                        }
                        show_playlist_modal.set(false);
                        active_menu_track.set(None);
                    },
                    on_create_playlist: move |name: String| {
                        if let Some(path) = selected_track_for_playlist.read().clone() {
                            let mut store = playlist_store.write();
                            store.playlists.push(crate::reader::models::Playlist {
                                id: uuid::Uuid::new_v4().to_string(),
                                name,
                                tracks: vec![path],
                            });
                        }
                        show_playlist_modal.set(false);
                        active_menu_track.set(None);
                    }
                }
            }

            div {
                class: "flex items-center justify-between mb-6",
                h1 { class: "text-3xl font-bold text-white", "Your Library" }
                button {
                    onclick: move |_| on_rescan.call(()),
                    class: "text-white/60 hover:text-white transition-colors p-2 rounded-full hover:bg-white/10",
                    title: "Rescan Library",
                    i { class: "fa-solid fa-rotate" }
                }
            }

            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-12",
                StatCard { label: "Tracks", value: "{lib.tracks.len()}", icon: "fa-music" }
                StatCard { label: "Albums", value: "{lib.albums.len()}", icon: "fa-compact-disc" }
                StatCard { label: "Artists", value: "{items.artist_count}", icon: "fa-user" }
                StatCard { label: "Playlists", value: "{playlist_store.read().playlists.len()}", icon: "fa-list" }
            }

            div {
                class: "flex items-center justify-between mb-4",
                h2 { class: "text-xl font-semibold text-white/80", "All Tracks" }
                div {
                    class: "flex space-x-1 bg-[#0A0A0A] border border-white/5 p-1 rounded-lg",
                    SortButton { active: *sort_order.read() == SortOrder::Title, label: "Title", onclick: move |_| sort_order.set(SortOrder::Title) }
                    SortButton { active: *sort_order.read() == SortOrder::Artist, label: "Artist", onclick: move |_| sort_order.set(SortOrder::Artist) }
                    SortButton { active: *sort_order.read() == SortOrder::Album, label: "Album", onclick: move |_| sort_order.set(SortOrder::Album) }
                }
            }
            div {
                class: "space-y-1 pb-20",
                if lib.tracks.is_empty() {
                    p { class: "text-slate-500 italic", "Scanning your music collection..." }
                } else {
                    {items.all_tracks.iter().map(|(track, cover_url)| {
                        let track_menu = track.clone();
                        let track_add = track.clone();
                        let track_key = track.path.display().to_string();
                        let is_menu_open = active_menu_track.read().as_ref() == Some(&track.path);
                        
                        rsx! {
                            TrackRow {
                                key: "{track_key}",
                                track: track.clone(),
                                cover_url: cover_url.clone(),
                                is_menu_open: is_menu_open,
                                on_click_menu: move |_| {
                                    if active_menu_track.read().as_ref() == Some(&track_menu.path) {
                                        active_menu_track.set(None);
                                    } else {
                                        active_menu_track.set(Some(track_menu.path.clone()));
                                    }
                                },
                                on_add_to_playlist: move |_| {
                                    selected_track_for_playlist.set(Some(track_add.path.clone()));
                                    show_playlist_modal.set(true);
                                    active_menu_track.set(None);
                                },
                                on_close_menu: move |_| active_menu_track.set(None)
                            }
                        }
                    })}
                }
            }
        }
    }
}


#[component]
fn SortButton(active: bool, label: &'static str, onclick: EventHandler) -> Element {
    rsx! {
        button {
            onclick: move |_| onclick.call(()),
            class: if active { "px-3 py-1 text-xs rounded-md bg-white/10 text-white font-medium transition-all" } else { "px-3 py-1 text-xs rounded-md text-white/40 hover:text-white/80 transition-all" },
            "{label}"
        }
    }
}
