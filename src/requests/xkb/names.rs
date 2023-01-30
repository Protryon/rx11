use std::collections::BTreeMap;

use crate::coding::xkb::{GetNamesRequest, GetNamesResponse};

use super::*;

#[derive(Clone, Debug)]
pub struct Names {
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub keycodes_name: Option<Atom>,
    pub geometry_name: Option<Atom>,
    pub symbols_name: Option<Atom>,
    pub phys_symbols_name: Option<Atom>,
    pub types_name: Option<Atom>,
    pub compat_name: Option<Atom>,
    pub key_type_names: Vec<Atom>,
    pub kt_level_names: Vec<Vec<Atom>>,
    pub indicator_names: BTreeMap<u32, Atom>,
    pub vmod_names: BTreeMap<VMod, Atom>,
    pub group_names: BTreeMap<Group, Atom>,
    pub key_names: BTreeMap<u8, String>,
    pub key_aliases: BTreeMap<String, String>,
    pub radio_group_names: Vec<Atom>,
}

impl X11Connection {
    pub(crate) async fn xkb_parse_names(&self, reply: GetNamesResponse) -> Result<Names> {
        let keycodes_name = self.maybe_get_atom_name(reply.values.keycodes_name_atom);
        let geometry_name = self.maybe_get_atom_name(reply.values.geometry_name_atom);
        let symbols_name = self.maybe_get_atom_name(reply.values.symbols_name_atom);
        let phys_symbols_name = self.maybe_get_atom_name(reply.values.phys_symbols_name_atom);
        let types_name = self.maybe_get_atom_name(reply.values.types_name_atom);
        let compat_name = self.maybe_get_atom_name(reply.values.compat_name_atom);

        let key_type_names = self.get_all_atoms(reply.values.key_type_name_atoms.unwrap_or_default());
        let kt_level_names = self.get_all_atoms(reply.values.kt_level_name_atoms.unwrap_or_default());
        let indicator_names = self.get_all_atoms(reply.values.indicator_name_atoms.unwrap_or_default());
        let vmod_names = self.get_all_atoms(reply.values.vmod_name_atoms.unwrap_or_default());
        let group_names = self.get_all_atoms(reply.values.group_name_atoms.unwrap_or_default());
        let radio_group_names = self.get_all_atoms(reply.values.radio_group_name_atoms.unwrap_or_default());

        Ok(Names {
            min_keycode: reply.min_keycode,
            max_keycode: reply.max_keycode,
            keycodes_name: keycodes_name.await?,
            geometry_name: geometry_name.await?,
            symbols_name: symbols_name.await?,
            phys_symbols_name: phys_symbols_name.await?,
            types_name: types_name.await?,
            compat_name: compat_name.await?,
            key_type_names: key_type_names.await?,
            kt_level_names: {
                let mut kt_level_names = kt_level_names.await?;
                kt_level_names.reverse();
                let mut out = vec![];
                for window in reply.values.num_levels_per_type.unwrap_or_default() {
                    out.push(kt_level_names.drain(0..window as usize).collect());
                }
                out
            },
            indicator_names: {
                println!("names indicators: {} {:08X}", reply.indicators, reply.indicators);
                let mut indicator_names = indicator_names.await?.into_iter();
                let mut output = BTreeMap::new();
                for bit in 0..31 {
                    if reply.indicators & (1 << bit) != 0 {
                        output.insert(bit, indicator_names.next().ok_or_else(|| anyhow!("missing indicator name"))?);
                    }
                }
                output
            },
            vmod_names: {
                let mut vmod_names = vmod_names.await?.into_iter();
                let mut output = BTreeMap::new();
                for bit in 0..15 {
                    if reply.vmods.0 & (1 << bit) != 0 {
                        output.insert(VMod(1 << bit), vmod_names.next().ok_or_else(|| anyhow!("missing vmod name"))?);
                    }
                }
                output
            },
            group_names: {
                let mut group_names = group_names.await?.into_iter();
                let mut output = BTreeMap::new();
                for bit in 0..7 {
                    if reply.group_names.0 & (1 << bit) != 0 {
                        output.insert(Group::from_repr(bit + 1)?, group_names.next().ok_or_else(|| anyhow!("missing group name"))?);
                    }
                }
                output
            },
            key_names: {
                let mut output = BTreeMap::new();
                for (i, name) in reply.values.key_names.unwrap_or_default().into_iter().enumerate() {
                    output.insert(i as u8 + reply.first_key, name);
                }
                output
            },
            key_aliases: {
                let mut output = BTreeMap::new();
                for name in reply.values.key_aliases.unwrap_or_default().into_iter() {
                    output.insert(name.alias, name.real);
                }
                output
            },
            radio_group_names: radio_group_names.await?,
        })
    }

    pub async fn xkb_get_names(&self, device: DeviceSpec, which: NameDetail) -> Result<Names> {
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetNames,
            GetNamesResponse,
            GetNamesRequest {
                device_spec: device.into(),
                which: which,
            }
        )
        .into_inner();

        self.xkb_parse_names(reply).await
    }

    //TODO: pub async fn xkb_set_names(&self, device: DeviceSpec, names: Names) -> Result<()>;
}
