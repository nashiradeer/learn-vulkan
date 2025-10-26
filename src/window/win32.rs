use std::{
    ffi::NulError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

use windows::{
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::{
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExA, DefWindowProcA,
            DispatchMessageA, GetMessageA, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage,
            RegisterClassA, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSA, WS_OVERLAPPEDWINDOW,
            WS_VISIBLE,
        },
    },
    core::s,
};

pub use windows::core::Error as WindowsError;

pub type Result<T> = StdResult<T, Error>;

pub struct Context {
    _hwnd: HWND,
    _instance: HINSTANCE,
}

impl Context {
    pub fn new() -> Result<Self> {
        unsafe {
            let instance = GetModuleHandleA(None)?.into();
            let window_class = s!("LearnVulkan");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW).map_err(Error::from)?,
                hInstance: instance,
                lpszClassName: window_class,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(window_procedure),
                ..Default::default()
            };

            if RegisterClassA(&wc) == 0 {
                return Err(Error::RegisterClassFailed);
            }

            let hwnd = CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                window_class,
                s!("Learn Vulkan"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                None,
                None,
            )
            .map_err(Error::from)?;

            Ok(Self {
                _hwnd: hwnd,
                _instance: instance,
            })
        }
    }

    pub fn message_loop(&self) {
        let mut msg = MSG::default();
        unsafe {
            while GetMessageA(&mut msg, None, 0, 0).into() {
                DispatchMessageA(&msg);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Windows(WindowsError),
    Nul(NulError),
    RegisterClassFailed,
}

impl From<windows::core::Error> for Error {
    fn from(err: windows::core::Error) -> Self {
        Error::Windows(err)
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Error::Nul(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::Windows(err) => write!(f, "Windows Error: {}", err),
            Error::Nul(err) => write!(f, "Nul Error: {}", err),
            Error::RegisterClassFailed => write!(f, "Failed to register window class"),
        }
    }
}

extern "system" fn window_procedure(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
    }
}
