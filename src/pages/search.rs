use crate::reader::Library;
use dioxus::prelude::*;

use crate::hooks::use_search_data::use_search_data;

#[component]
pub fn Search(library: Signal<Library>) -> Element {
    let mut data = use_search_data(library);

    rsx! {
        div {
            class: "p-8",
            h1 { class: "text-3xl font-bold text-white mb-6", "Search" }

            div {
                class: "relative max-w-2xl",
                i { class: "fa-solid fa-magnifying-glass absolute left-4 top-1/2 -translate-y-1/2 text-slate-500" }
                input {
                    r#type: "text",
                    placeholder: "Search for artists, albums or tracks...",
                    class: "w-full bg-white/5 border border-white/10 rounded-full py-3 pl-12 pr-4 text-white focus:outline-none focus:border-white/20 transition-colors",
                    value: "{data.search_query}",
                    oninput: move |evt| data.search_query.set(evt.value())
                }
            }

            if let Some((tracks, albums)) = (data.search_results)() {
                div { class: "mt-8 space-y-8",
                    if !tracks.is_empty() {
                        div {
                            h2 { class: "text-xl font-semibold text-white/80 mb-4", "Tracks" }
                            div { class: "space-y-2",
                                for (track, cover_url) in &tracks {
                                    div {
                                        key: "{track.path.display()}",
                                        class: "flex items-center gap-4 p-3 bg-white/5 rounded-lg hover:bg-white/10 cursor-pointer group",
                                        if let Some(url) = cover_url {
                                            img {
                                                src: "{url}",
                                                class: "w-10 h-10 rounded object-cover",
                                                loading: "lazy",
                                                decoding: "async",
                                            }
                                        } else {
                                            div { class: "w-10 h-10 bg-white/10 rounded flex items-center justify-center text-slate-400 group-hover:text-white",
                                                i { class: "fa-solid fa-music" }
                                            }
                                        }
                                        div { class: "flex-1",
                                            h3 { class: "text-white font-medium truncate", "{track.title}" }
                                            p { class: "text-sm text-slate-400 truncate", "{track.artist} - {track.album}" }
                                        }
                                        span { class: "text-xs text-slate-500",
                                            {format!("{}:{:02}", track.duration / 60, track.duration % 60)}
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !albums.is_empty() {
                        div {
                            h2 { class: "text-xl font-semibold text-white/80 mb-4", "Albums" }
                            div { class: "grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-4",
                                for (album, cover_url) in &albums {
                                    div {
                                        key: "{album.id}",
                                        class: "p-4 bg-white/5 rounded-xl hover:bg-white/10 transition-colors cursor-pointer group",
                                        div {
                                            class: "aspect-square rounded-lg bg-black/40 mb-3 overflow-hidden shadow-lg relative",
                                            if let Some(url) = cover_url {
                                                img {
                                                    src: "{url}",
                                                    class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                                                    loading: "lazy",
                                                    decoding: "async",
                                                }
                                            } else {
                                                div { class: "w-full h-full flex items-center justify-center",
                                                    i { class: "fa-solid fa-compact-disc text-4xl text-white/20" }
                                                }
                                            }
                                        }
                                        h3 { class: "text-white font-medium truncate", "{album.title}" }
                                        p { class: "text-sm text-slate-400 truncate", "{album.artist}" }
                                    }
                                }
                            }
                        }
                    }
                    
                    if tracks.is_empty() && albums.is_empty() {
                        div { class: "text-center py-12 text-slate-500",
                            p { "No results found for \"{data.search_query}\"" }
                        }
                    }
                }
            } else {
                div { class: "mt-12",
                    h2 { class: "text-xl font-semibold text-white/80 mb-4", "Browse Genres" }
                    if (data.genres)().is_empty() {
                        p { class: "text-slate-500 italic", "No genres found in your library." }
                    } else {
                        div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                            for (genre, cover_url) in (data.genres)() {
                                div {
                                    key: "{genre}",
                                    class: "aspect-video bg-gradient-to-br from-indigo-600 to-purple-700 rounded-xl p-4 cursor-pointer hover:scale-[1.02] transition-transform flex items-end relative overflow-hidden group content-visibility-auto",
                                    if let Some(url) = cover_url {
                                        img {
                                            src: "{url}",
                                            class: "absolute inset-0 w-full h-full object-cover opacity-60 group-hover:scale-110 transition-transform duration-500 will-change-transform",
                                            loading: "lazy",
                                            decoding: "async",
                                        }
                                        div { class: "absolute inset-0 bg-gradient-to-t from-black/90 via-black/40 to-transparent" }
                                    }
                                    span { class: "text-lg font-bold text-white relative z-10", "{genre}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
