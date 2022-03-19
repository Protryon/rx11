#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl From<crate::coding::x11::Rectangle> for Rectangle {
    fn from(from: crate::coding::x11::Rectangle) -> Self {
        Self {
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
        }
    }
}

impl Into<crate::coding::x11::Rectangle> for Rectangle {
    fn into(self) -> crate::coding::x11::Rectangle {
        crate::coding::x11::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl From<crate::coding::xfixes::Rectangle> for Rectangle {
    fn from(from: crate::coding::xfixes::Rectangle) -> Self {
        Self {
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
        }
    }
}

impl Into<crate::coding::xfixes::Rectangle> for Rectangle {
    fn into(self) -> crate::coding::xfixes::Rectangle {
        crate::coding::xfixes::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

impl From<crate::coding::shape::Rectangle> for Rectangle {
    fn from(from: crate::coding::shape::Rectangle) -> Self {
        Self {
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
        }
    }
}

impl Into<crate::coding::shape::Rectangle> for Rectangle {
    fn into(self) -> crate::coding::shape::Rectangle {
        crate::coding::shape::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}
