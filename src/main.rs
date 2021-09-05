mod bindings {
    windows::include_bindings!();
}
use std::ffi::c_void;

use bindings::{
    Windows::Win32::Foundation::{HWND, LPARAM, LRESULT, PSTR, PWSTR, RECT, WPARAM},
    Windows::Win32::Graphics::Direct2D::{
        D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D_SIZE_U, D2D1_COLOR_F, D2D_RECT_F, ID2D1Brush, D2D1_RENDER_TARGET_PROPERTIES,
        D2D1CreateFactory,ID2D1Factory,D2D1_FACTORY_TYPE_SINGLE_THREADED
    },
    Windows::Win32::System::LibraryLoader::GetModuleHandleW,
    Windows::Win32::UI::WindowsAndMessaging::{
        self, CreateWindowExA, DefWindowProcA, DispatchMessageA, GetClientRect, GetMessageA, PostQuitMessage, RegisterClassExA, CW_USEDEFAULT, MSG, WNDCLASSEXA, WS_OVERLAPPEDWINDOW,
        WS_VISIBLE,
    },
};
use windows::Interface;


unsafe extern "system" fn WindowProc(
    hwnd: HWND,
    umsg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match umsg {
        WindowsAndMessaging::WM_CREATE => {
            let mut factory : *mut ID2D1Factory = std::ptr::null_mut() as _;
            
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                &<ID2D1Factory as Interface>::IID,
                std::ptr::null(),
                (&mut factory) as *mut _ as *mut *mut c_void,
            );

            let mut rc = RECT::default();
            GetClientRect(hwnd, &mut rc );

            let hwnd_render_props = {
                let mut out = D2D1_HWND_RENDER_TARGET_PROPERTIES::default();
                out.hwnd = hwnd;
                out.pixelSize = D2D_SIZE_U { 
                    width : (rc.right - rc.left) as u32,
                    height : (rc.bottom - rc.top) as u32,
                };
                out
            };
            let rt_props = D2D1_RENDER_TARGET_PROPERTIES::default();
            println!("CREATE RENDERING TARGET!");
            let rt = match factory.as_ref().unwrap().CreateHwndRenderTarget(&rt_props, &hwnd_render_props) {
                Ok(rt) => {println!("Render target Created"); rt },
                Err(err) => {println!("{:?}",err); return LRESULT(1); }
            };
            
            
            let black = D2D1_COLOR_F { r:0.0, g:0.0, b:0.0, a:1.0 };
            let black_brush =  rt.CreateSolidColorBrush(&black,std::ptr::null()).expect("could not create brush");
            
            rt.BeginDraw();
            
            let target = D2D_RECT_F {
                left : (rc.left + 100) as f32,
                top : (rc.top + 100) as f32,
                right : (rc.right - 100) as f32,
                bottom : (rc.bottom - 100) as f32,
            };
            
            rt.DrawRectangle(&target as *const _, black_brush, 10.00, None);
            
            let mut one =0;
            let mut two =0; 
            
            rt.EndDraw(&mut one, &mut two).expect("Faild to end drawing.");
            
            LRESULT(0)
        },
        WindowsAndMessaging::WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        },
        _ => {
            DefWindowProcA(hwnd, umsg, wparam, lparam)
        },
    }
}

pub fn main() {
    let hinstance = unsafe { GetModuleHandleW(PWSTR::NULL) };
    let mut cname = String::from("TestingClass\0");
    let wc = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        lpszClassName: PSTR(cname.as_mut_ptr()),
        lpfnWndProc: Some(WindowProc),
        hInstance: hinstance,
        ..Default::default()
    };
    unsafe { RegisterClassExA(&wc) };
    let mut text = String::from("Learning Mode ON\0");
    let hwnd = unsafe {
        CreateWindowExA(
            Default::default(),
            "TestingClass\0",
            PSTR(text.as_mut_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            hinstance,
            std::ptr::null_mut(),
        )
    };
    if hwnd.is_null() {
        panic!("Failed to crate window");
    }
    let mut msg = MSG::default();
    while unsafe { GetMessageA(&mut msg, HWND(0), 0, 0) }.as_bool() {
        unsafe {
            DispatchMessageA(&msg);
        }
    }
}
