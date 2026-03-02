use std::sync::OnceLock;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use windows::{
    Foundation::{TimeSpan, TypedEventHandler, Uri},
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs,
        SystemMediaTransportControls, SystemMediaTransportControlsButton,
        SystemMediaTransportControlsButtonPressedEventArgs,
        SystemMediaTransportControlsTimelineProperties,
    },
    Storage::Streams::RandomAccessStreamReference,
    Win32::{
        Foundation::{HWND, LPARAM},
        System::Threading::GetCurrentProcessId,
        System::WinRT::RoGetActivationFactory,
        UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible},
    },
};
use windows::core::{BOOL, HSTRING, Ref};

#[derive(Debug)]
pub enum SystemEvent {
    Play,
    Pause,
    Toggle,
    Next,
    Prev,
    Seek(f64),
}

static EVENT_SENDER: OnceLock<UnboundedSender<SystemEvent>> = OnceLock::new();
static EVENT_RECEIVER: OnceLock<Mutex<UnboundedReceiver<SystemEvent>>> = OnceLock::new();
static SMTC: OnceLock<SystemMediaTransportControls> = OnceLock::new();

fn get_tx() -> UnboundedSender<SystemEvent> {
    EVENT_SENDER
        .get_or_init(|| {
            let (tx, rx) = mpsc::unbounded_channel();
            let _ = EVENT_RECEIVER.set(Mutex::new(rx));
            tx
        })
        .clone()
}

pub fn poll_event() -> Option<SystemEvent> {
    EVENT_RECEIVER.get()?.try_lock().ok()?.try_recv().ok()
}

pub async fn wait_event() -> Option<SystemEvent> {
    if let Some(rx) = EVENT_RECEIVER.get() {
        let mut guard = rx.lock().await;
        guard.recv().await
    } else {
        None
    }
}

// HWND discovery
struct EnumData {
    pid: u32,
    hwnd: HWND,
}

unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = unsafe { &mut *(lparam.0 as *mut EnumData) };
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    if pid == data.pid && unsafe { IsWindowVisible(hwnd).as_bool() } {
        data.hwnd = hwnd;
        BOOL(0) // stop enumeration
    } else {
        BOOL(1)
    }
}

fn find_main_hwnd() -> Option<HWND> {
    let mut data = EnumData {
        pid: unsafe { GetCurrentProcessId() },
        hwnd: HWND(std::ptr::null_mut()),
    };

    // return Err even on success
    let _ = unsafe {
        EnumWindows(
            Some(enum_proc),
            LPARAM(&mut data as *mut EnumData as isize),
        )
    };
    (!data.hwnd.0.is_null()).then_some(data.hwnd)
}

// SMTC setup
use windows::Win32::System::WinRT::ISystemMediaTransportControlsInterop;

fn setup_smtc(hwnd: HWND) {
    if SMTC.get().is_some() {
        return;
    }

    let result = (|| unsafe {
        let class_id = HSTRING::from("Windows.Media.SystemMediaTransportControls");
        let interop: ISystemMediaTransportControlsInterop = RoGetActivationFactory(&class_id)?;
        let smtc: SystemMediaTransportControls = interop.GetForWindow(hwnd)?;

        smtc.SetIsEnabled(true)?;
        smtc.SetIsPlayEnabled(true)?;
        smtc.SetIsPauseEnabled(true)?;
        smtc.SetIsNextEnabled(true)?;
        smtc.SetIsPreviousEnabled(true)?;
        smtc.SetIsStopEnabled(true)?;

        let tx = get_tx();
        smtc.ButtonPressed(&TypedEventHandler::new(
            move |_: Ref<SystemMediaTransportControls>,
                  args: Ref<SystemMediaTransportControlsButtonPressedEventArgs>|
                  -> windows::core::Result<()> {
                if let Some(args) = args.as_ref() {
                    let btn: SystemMediaTransportControlsButton = args.Button()?;
                    let evt = if btn == SystemMediaTransportControlsButton::Play {
                        Some(SystemEvent::Play)
                    } else if btn == SystemMediaTransportControlsButton::Pause {
                        Some(SystemEvent::Pause)
                    } else if btn == SystemMediaTransportControlsButton::Next {
                        Some(SystemEvent::Next)
                    } else if btn == SystemMediaTransportControlsButton::Previous {
                        Some(SystemEvent::Prev)
                    } else {
                        None
                    };
                    if let Some(e) = evt {
                        let _ = tx.send(e);
                    }
                }
                Ok(())
            },
        ))?;

        let tx_seek = get_tx();
        smtc.PlaybackPositionChangeRequested(&TypedEventHandler::new(
            move |_: Ref<SystemMediaTransportControls>,
                  args: Ref<PlaybackPositionChangeRequestedEventArgs>|
                  -> windows::core::Result<()> {
                if let Some(args) = args.as_ref() {
                    let pos: TimeSpan = args.RequestedPlaybackPosition()?;
                    let secs = pos.Duration as f64 / 1e7;  // TimeSpan::Duration is in 100-nanosecond ticks
                    let _ = tx_seek.send(SystemEvent::Seek(secs));
                }
                Ok(())
            },
        ))?;

        windows::core::Result::Ok(smtc)
    })();

    match result {
        Ok(smtc) => {
            if SMTC.set(smtc).is_ok() {
                println!("[windows] SMTC initialised");
            }
        }
        Err(e) => eprintln!("[windows] SMTC setup failed: {e:?}"),
    }
}

pub fn init() {
    if SMTC.get().is_some() {
        return;
    }
    match find_main_hwnd() {
        Some(hwnd) => setup_smtc(hwnd),
        None => eprintln!("[windows] Could not find main HWND for SMTC"),
    }
}

pub fn update_now_playing(
    title: &str,
    artist: &str,
    album: &str,
    duration: f64,
    position: f64,
    playing: bool,
    artwork_path: Option<&str>,
) {
    // init in case init() wasn't called before the first track plays
    if SMTC.get().is_none() {
        if let Some(hwnd) = find_main_hwnd() {
            setup_smtc(hwnd);
        }
    }

    let Some(smtc) = SMTC.get() else { return };

    let _ = smtc.SetPlaybackStatus(if playing {
        MediaPlaybackStatus::Playing
    } else {
        MediaPlaybackStatus::Paused
    });

    if let Ok(updater) = smtc.DisplayUpdater() {
        let _ = updater.SetType(MediaPlaybackType::Music);
        if let Ok(props) = updater.MusicProperties() {
            let _ = props.SetTitle(&HSTRING::from(title));
            let _ = props.SetArtist(&HSTRING::from(artist));
            let _ = props.SetAlbumTitle(&HSTRING::from(album));
        }

        if let Some(path) = artwork_path {
            // Pass http:// urls through, converts local paths to a file:/// URI
            let uri_str = if path.starts_with("http://") || path.starts_with("https://") {
                path.to_string()
            } else {
                format!("file:///{}", path.replace('\\', "/"))
            };
            if let Ok(uri) = Uri::CreateUri(&HSTRING::from(uri_str)) {
                if let Ok(stream_ref) = RandomAccessStreamReference::CreateFromUri(&uri) {
                    let _ = updater.SetThumbnail(&stream_ref);
                }
            }
        }

        let _ = updater.Update();
    }

    // Push timeline for a proper scrub bar
    if duration > 0.0 {
        if let Ok(tl) = SystemMediaTransportControlsTimelineProperties::new() {
            let ticks = |secs: f64| TimeSpan { Duration: (secs * 1e7) as i64 };
            let _ = tl.SetStartTime(ticks(0.0));
            let _ = tl.SetMinSeekTime(ticks(0.0));
            let _ = tl.SetPosition(ticks(position));
            let _ = tl.SetMaxSeekTime(ticks(duration));
            let _ = tl.SetEndTime(ticks(duration));
            let _ = smtc.UpdateTimelineProperties(&tl);
        }
    }
}