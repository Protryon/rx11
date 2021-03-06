pub mod x11 {
    protospec::include_spec!("x11");
}
pub use x11::*;

impl RequestBody {
    pub fn is_void(&self) -> bool {
        match self {
            RequestBody::CreateWindow(_) => true,
            RequestBody::ChangeWindowAttributes(_) => true,
            RequestBody::GetWindowAttributes(_) => false,
            RequestBody::DestroyWindow(_) => true,
            RequestBody::DestroySubwindows(_) => true,
            RequestBody::ChangeSaveSet(_) => true,
            RequestBody::ReparentWindow(_) => true,
            RequestBody::MapWindow(_) => true,
            RequestBody::MapSubwindows(_) => true,
            RequestBody::UnmapWindow(_) => true,
            RequestBody::UnmapSubwindows(_) => true,
            RequestBody::ConfigureWindow(_) => true,
            RequestBody::CirculateWindow(_) => true,
            RequestBody::GetGeometry(_) => false,
            RequestBody::QueryTree(_) => false,
            RequestBody::InternAtom(_) => false,
            RequestBody::GetAtomName(_) => false,
            RequestBody::ChangeProperty(_) => true,
            RequestBody::DeleteProperty(_) => true,
            RequestBody::GetProperty(_) => false,
            RequestBody::ListProperties(_) => false,
            RequestBody::SetSelectionOwner(_) => true,
            RequestBody::GetSelectionOwner(_) => false,
            RequestBody::ConvertSelection(_) => true,
            RequestBody::SendEvent(_) => true,
            RequestBody::GrabPointer(_) => false,
            RequestBody::UngrabPointer(_) => true,
            RequestBody::GrabButton(_) => true,
            RequestBody::UngrabButton(_) => true,
            RequestBody::ChangeActivePointerGrab(_) => true,
            RequestBody::GrabKeyboard(_) => false,
            RequestBody::UngrabKeyboard(_) => true,
            RequestBody::GrabKey(_) => true,
            RequestBody::UngrabKey(_) => true,
            RequestBody::AllowEvents(_) => true,
            RequestBody::GrabServer(_) => true,
            RequestBody::UngrabServer(_) => true,
            RequestBody::QueryPointer(_) => false,
            RequestBody::GetMotionEvents(_) => true,
            RequestBody::TranslateCoordinates(_) => false,
            RequestBody::WarpPointer(_) => true,
            RequestBody::SetInputFocus(_) => true,
            RequestBody::GetInputFocus(_) => false,
            RequestBody::QueryKeymap(_) => false,
            RequestBody::OpenFont(_) => true,
            RequestBody::CloseFont(_) => true,
            RequestBody::QueryFont(_) => false,
            RequestBody::QueryTextExtents(_) => false,
            RequestBody::ListFonts(_) => false,
            RequestBody::ListFontsWithInfo(_) => false,
            RequestBody::SetFontPath(_) => true,
            RequestBody::GetFontPath(_) => false,
            RequestBody::CreatePixmap(_) => true,
            RequestBody::FreePixmap(_) => true,
            RequestBody::CreateGC(_) => true,
            RequestBody::ChangeGC(_) => true,
            RequestBody::CopyGC(_) => true,
            RequestBody::SetDashes(_) => true,
            RequestBody::SetClipRectangles(_) => true,
            RequestBody::FreeGC(_) => true,
            RequestBody::ClearArea(_) => true,
            RequestBody::CopyArea(_) => true,
            RequestBody::CopyPlane(_) => true,
            RequestBody::PolyPoint(_) => true,
            RequestBody::PolyLine(_) => true,
            RequestBody::PolySegment(_) => true,
            RequestBody::PolyRectangle(_) => true,
            RequestBody::PolyArc(_) => true,
            RequestBody::FillPoly(_) => true,
            RequestBody::PolyFillRectangle(_) => true,
            RequestBody::PolyFillArc(_) => true,
            RequestBody::PutImage(_) => true,
            RequestBody::GetImage(_) => false,
            RequestBody::PolyText8(_) => true,
            RequestBody::PolyText16(_) => true,
            RequestBody::ImageText8(_) => true,
            RequestBody::ImageText16(_) => true,
            RequestBody::CreateColormap(_) => true,
            RequestBody::FreeColormap(_) => true,
            RequestBody::CopyColormapAndFree(_) => true,
            RequestBody::InstallColormap(_) => true,
            RequestBody::UninstallColormap(_) => true,
            RequestBody::ListInstalledColormaps(_) => false,
            RequestBody::AllocColor(_) => false,
            RequestBody::AllocNamedColor(_) => false,
            RequestBody::AllocColorCells(_) => false,
            RequestBody::AllocColorPlanes(_) => false,
            RequestBody::FreeColors(_) => true,
            RequestBody::StoreColors(_) => true,
            RequestBody::StoreNamedColor(_) => true,
            RequestBody::QueryColors(_) => false,
            RequestBody::LookupColor(_) => false,
            RequestBody::CreateCursor(_) => true,
            RequestBody::CreateGlyphCursor(_) => true,
            RequestBody::FreeCursor(_) => true,
            RequestBody::RecolorCursor(_) => true,
            RequestBody::QueryBestSize(_) => false,
            RequestBody::QueryExtension(_) => false,
            RequestBody::ListExtensions(_) => false,
            RequestBody::ChangeKeyboardMapping(_) => true,
            RequestBody::GetKeyboardMapping(_) => false,
            RequestBody::ChangeKeyboardControl(_) => true,
            RequestBody::GetKeyboardControl(_) => false,
            RequestBody::Bell(_) => true,
            RequestBody::ChangePointerControl(_) => true,
            RequestBody::GetPointerControl(_) => false,
            RequestBody::SetScreenSaver(_) => true,
            RequestBody::GetScreenSaver(_) => false,
            RequestBody::ChangeHosts(_) => true,
            RequestBody::ListHosts(_) => false,
            RequestBody::SetAccessControl(_) => true,
            RequestBody::SetCloseDownMode(_) => true,
            RequestBody::KillClient(_) => true,
            RequestBody::RotateProperties(_) => true,
            RequestBody::ForceScreenSaver(_) => true,
            RequestBody::SetPointerMapping(_) => false,
            RequestBody::GetPointerMapping(_) => false,
            RequestBody::SetModifierMapping(_) => false,
            RequestBody::GetModifierMapping(_) => false,
            RequestBody::NoOperation(_) => true,
            RequestBody::Ext(_) => unimplemented!(),
        }
    }
}
