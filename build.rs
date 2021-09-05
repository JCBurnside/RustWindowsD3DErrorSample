fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::*,
        Windows::Win32::System::LibraryLoader::GetModuleHandleW,
        Windows::Win32::Graphics::Direct2D::{ID2D1Factory,D2D1CreateFactory, D2D1_COLOR_F,ID2D1HwndRenderTarget,ID2D1SolidColorBrush},
    }
}
