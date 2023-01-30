use std::collections::BTreeMap;

use super::*;

use crate::coding::xkb::{DoodadType, GetGeometryRequest, GetGeometryResponse};
pub use crate::coding::xkb::{Key, Outline, Overlay, OverlayKey, OverlayRow, Point, Row};

#[derive(Debug, Clone)]
pub struct GeometryData {
    pub name: Atom,
    pub width_mm: u16,
    pub height_mm: u16,
    pub base_color_index: u8,
    pub label_color_index: u8,
    pub label_font: String,
    pub properties: BTreeMap<String, String>,
    pub colors: Vec<String>,
    pub shapes: Vec<Shape>,
    pub sections: Vec<Section>,
    pub doodads: Vec<Doodad>,
    pub key_aliases: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub name: Atom,
    pub primary_index: u8,
    pub approx_index: u8,
    pub outlines: Vec<Outline>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: Atom,
    pub top: i16,
    pub left: i16,
    pub width: u16,
    pub height: u16,
    pub angle: i16,
    pub priority: u8,
    pub rows: Vec<Row>,
    pub doodads: Vec<Doodad>,
    pub overlays: Vec<Overlay>,
}

#[derive(Debug, Clone)]
pub enum DoodadData {
    Outline {
        color_index: u8,
        shape_index: u8,
    },
    Solid {
        color_index: u8,
        shape_index: u8,
    },
    Text {
        width: u16,
        height: u16,
        color_index: u8,
        text: String,
        font: String,
    },
    Indicator {
        shape_index: u8,
        on_color_index: u8,
        off_color_index: u8,
    },
    Logo {
        color_index: u8,
        shape_index: u8,
        name: String,
    },
}

#[derive(Debug, Clone)]
pub struct Doodad {
    pub name: Atom,
    pub priority: u8,
    pub top: i16,
    pub left: i16,
    pub angle: i16,
    pub data: DoodadData,
}

impl X11Connection {
    async fn convert_doodad(&self, from: crate::coding::xkb::Doodad) -> Result<Doodad> {
        use crate::coding::xkb::DoodadData::*;
        Ok(Doodad {
            name: self.get_atom_name(from.name_atom).await?,
            priority: from.priority,
            top: from.top,
            left: from.left,
            angle: from.angle,
            data: match from.data {
                Shape {
                    color_index,
                    shape_index,
                } => match from.type_ {
                    DoodadType::Outline => DoodadData::Outline {
                        color_index,
                        shape_index,
                    },
                    DoodadType::Solid => DoodadData::Solid {
                        color_index,
                        shape_index,
                    },
                    _ => unimplemented!(),
                },
                Text {
                    width,
                    height,
                    color_index,
                    text,
                    font,
                } => DoodadData::Text {
                    width,
                    height,
                    color_index,
                    text: text.string,
                    font: font.string,
                },
                Indicator {
                    shape_index,
                    on_color_index,
                    off_color_index,
                } => DoodadData::Indicator {
                    shape_index,
                    on_color_index,
                    off_color_index,
                },
                Logo {
                    color_index,
                    shape_index,
                    logo_name,
                } => DoodadData::Logo {
                    color_index,
                    shape_index,
                    name: logo_name.string,
                },
            },
        })
    }

    pub(crate) async fn xkb_parse_geometry(&self, reply: GetGeometryResponse) -> Result<GeometryData> {
        Ok(GeometryData {
            name: self.get_atom_name(reply.name_atom).await?,
            width_mm: reply.width_mm,
            height_mm: reply.height_mm,
            base_color_index: reply.base_color_index,
            label_color_index: reply.label_color_index,
            label_font: reply.label_font.string,
            properties: reply.properties.into_iter().map(|x| (x.name.string, x.value.string)).collect(),
            colors: reply.colors.into_iter().map(|x| x.string).collect(),
            shapes: {
                let mut out = vec![];
                for shape in reply.shapes {
                    out.push(Shape {
                        name: self.get_atom_name(shape.name_atom).await?,
                        primary_index: shape.primary_index,
                        approx_index: shape.approx_index,
                        outlines: shape.outlines,
                    });
                }
                out
            },
            sections: {
                let mut out = vec![];
                for section in reply.sections {
                    out.push(Section {
                        name: self.get_atom_name(section.name_atom).await?,
                        top: section.top,
                        left: section.left,
                        width: section.width,
                        height: section.height,
                        angle: section.angle,
                        priority: section.priority,
                        rows: section.rows,
                        doodads: {
                            let mut out = vec![];
                            for doodad in section.doodads {
                                out.push(self.convert_doodad(doodad).await?);
                            }
                            out
                        },
                        overlays: section.overlays,
                    });
                }
                out
            },
            doodads: {
                let mut out = vec![];
                for doodad in reply.doodads {
                    out.push(self.convert_doodad(doodad).await?);
                }
                out
            },
            key_aliases: reply.key_aliases.into_iter().map(|x| (x.alias, x.real)).collect(),
        })
    }

    pub async fn xkb_get_geometry(&self, device: DeviceSpec, name: Atom) -> Result<GeometryData> {
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetGeometry,
            GetGeometryResponse,
            GetGeometryRequest {
                device_spec: device.into(),
                name_atom: name.handle,
            }
        )
        .into_inner();

        self.xkb_parse_geometry(reply).await
    }

    //TODO: pub async fn xkb_set_geometry(&self, device: DeviceSpec) -> Result<()>;
}
