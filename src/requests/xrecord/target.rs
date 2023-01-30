use anyhow::Context;

use crate::{
    coding::xrecord::{ExtRange, Range, Range16, Range8},
    net::Extension,
};

use super::*;

#[derive(Clone, Copy, Debug)]
pub enum RecordTarget {
    CoreRequest(MajorOpcode),
    CoreReply(MajorOpcode),
    XgeRequest,
    XgeReply,
    ShapeRequest(ShapeOpcode),
    ShapeReply(ShapeOpcode),
    XFixesRequest(XFOpcode),
    XFixesReply(XFOpcode),
    XInputRequest(XIOpcode),
    XInputReply(XIOpcode),
    XKBRequest(XKBOpcode),
    XKBReply(XKBOpcode),
    XRandrRequest(XROpcode),
    XRandrReply(XROpcode),
    XRecordRequest(XRecordOpcode),
    XRecordReply(XRecordOpcode),
    DeliveredCoreEvent(EventCode),
    DeviceCoreEvent(EventCode),
    DeliveredXkbEvent,
    DeviceXkbEvent,
    DeliveredXFixesEvent(XFEventCode),
    DeviceXFixesEvent(XFEventCode),
    DeliveredXRandrEvent(XREventCode),
    DeviceXRandrEvent(XREventCode),
    DeliveredShapeEvent(ShapeEventCode),
    DeviceShapeEvent(ShapeEventCode),
    DeliveredXInputEvent(XIEventCode),
    DeviceXInputEvent(XIEventCode),

    CoreError(ErrorCode),
    XkbError,
    XInputError,
    XFixesError,
    XRecordError,

    ClientStarted,
    ClientDied,
}

impl RecordTarget {
    pub(crate) fn from_targets(conn: &X11Connection, targets: impl IntoIterator<Item = Range>) -> Result<Vec<RecordTarget>> {
        let mut out = vec![];
        for target in targets {
            for major in target.core_requests.first..=target.core_requests.last {
                out.push(RecordTarget::CoreRequest(MajorOpcode::from_repr(major)?));
            }
            for major in target.core_replies.first..=target.core_replies.last {
                out.push(RecordTarget::CoreReply(MajorOpcode::from_repr(major)?));
            }
            for major in target.ext_requests.major.first..=target.ext_requests.major.last {
                let ext = conn.get_ext_info_by_opcode(major).context("unknown extension")?;
                for minor in target.ext_requests.minor.first..=target.ext_requests.minor.last {
                    out.push(match ext.extension {
                        Extension::Xge => RecordTarget::XgeRequest,
                        Extension::Shape => RecordTarget::ShapeRequest(ShapeOpcode::from_repr(minor as u8)?),
                        Extension::XFixes => RecordTarget::XFixesRequest(XFOpcode::from_repr(minor as u8)?),
                        Extension::XInput => RecordTarget::XInputRequest(XIOpcode::from_repr(minor as u8)?),
                        Extension::XKB => RecordTarget::XKBRequest(XKBOpcode::from_repr(minor as u8)?),
                        Extension::XRandr => RecordTarget::XRandrRequest(XROpcode::from_repr(minor as u8)?),
                        Extension::XRecord => RecordTarget::XRecordRequest(XRecordOpcode::from_repr(minor as u8)?),
                        Extension::Unknown => bail!("unknown extension encountered"),
                    });
                }
            }
            for major in target.ext_replies.major.first..=target.ext_replies.major.last {
                let ext = conn.get_ext_info_by_opcode(major).context("unknown extension")?;
                for minor in target.ext_replies.minor.first..=target.ext_replies.minor.last {
                    out.push(match ext.extension {
                        Extension::Xge => RecordTarget::XgeReply,
                        Extension::Shape => RecordTarget::ShapeReply(ShapeOpcode::from_repr(minor as u8)?),
                        Extension::XFixes => RecordTarget::XFixesReply(XFOpcode::from_repr(minor as u8)?),
                        Extension::XInput => RecordTarget::XInputReply(XIOpcode::from_repr(minor as u8)?),
                        Extension::XKB => RecordTarget::XKBReply(XKBOpcode::from_repr(minor as u8)?),
                        Extension::XRandr => RecordTarget::XRandrReply(XROpcode::from_repr(minor as u8)?),
                        Extension::XRecord => RecordTarget::XRecordReply(XRecordOpcode::from_repr(minor as u8)?),
                        Extension::Unknown => bail!("unknown extension encountered"),
                    });
                }
            }
            for code in target.delivered_events.first..=target.delivered_events.last {
                if code < 64 {
                    out.push(RecordTarget::DeliveredCoreEvent(EventCode::from_repr(code)?));
                    continue;
                }

                if let Some(xkb) = conn.get_ext_info(XKB_EXT_NAME) {
                    if code == xkb.event_start {
                        out.push(RecordTarget::DeliveredXkbEvent);
                        continue;
                    }
                }

                if let Some(xfixes) = conn.get_ext_info(XFIXES_EXT_NAME) {
                    if code >= xfixes.event_start && code < xfixes.event_start + xfixes.event_count {
                        out.push(RecordTarget::DeliveredXFixesEvent(XFEventCode::from_repr(code)?));
                        continue;
                    }
                }

                if let Some(xrandr) = conn.get_ext_info(XRANDR_EXT_NAME) {
                    if code >= xrandr.event_start && code < xrandr.event_start + xrandr.event_count {
                        out.push(RecordTarget::DeliveredXRandrEvent(XREventCode::from_repr(code)?));
                        continue;
                    }
                }

                if let Some(shape) = conn.get_ext_info(SHAPE_EXT_NAME) {
                    if code >= shape.event_start && code < shape.event_start + shape.event_count {
                        out.push(RecordTarget::DeliveredShapeEvent(ShapeEventCode::from_repr(code)?));
                        continue;
                    }
                }

                //TODO: XGE events?
            }
            for code in target.device_events.first..=target.device_events.last {
                if code < 64 {
                    out.push(RecordTarget::DeviceCoreEvent(EventCode::from_repr(code)?));
                    continue;
                }

                if let Some(xkb) = conn.get_ext_info(XKB_EXT_NAME) {
                    if code == xkb.event_start {
                        out.push(RecordTarget::DeviceXkbEvent);
                        continue;
                    }
                }

                if let Some(xfixes) = conn.get_ext_info(XFIXES_EXT_NAME) {
                    if code >= xfixes.event_start && code < xfixes.event_start + xfixes.event_count {
                        out.push(RecordTarget::DeviceXFixesEvent(XFEventCode::from_repr(code)?));
                        continue;
                    }
                }

                if let Some(xrandr) = conn.get_ext_info(XRANDR_EXT_NAME) {
                    if code >= xrandr.event_start && code < xrandr.event_start + xrandr.event_count {
                        out.push(RecordTarget::DeviceXRandrEvent(XREventCode::from_repr(code)?));
                        continue;
                    }
                }

                if let Some(shape) = conn.get_ext_info(SHAPE_EXT_NAME) {
                    if code >= shape.event_start && code < shape.event_start + shape.event_count {
                        out.push(RecordTarget::DeviceShapeEvent(ShapeEventCode::from_repr(code)?));
                        continue;
                    }
                }

                //TODO: XGE events?
            }
            for code in target.errors.first..=target.errors.last {
                if let Ok(code) = ErrorCode::from_repr(code) {
                    out.push(RecordTarget::CoreError(code));
                }
                if let Some(xkb) = conn.get_ext_info(XKB_EXT_NAME) {
                    if code == xkb.error_start {
                        out.push(RecordTarget::XkbError);
                    }
                }
                if let Some(xinput) = conn.get_ext_info(XINPUT_EXT_NAME) {
                    if code == xinput.error_start {
                        out.push(RecordTarget::XInputError);
                    }
                }
                if let Some(xfixes) = conn.get_ext_info(XFIXES_EXT_NAME) {
                    if code == xfixes.error_start {
                        out.push(RecordTarget::XFixesError);
                    }
                }
                if let Some(xrecord) = conn.get_ext_info(XRECORD_EXT_NAME) {
                    if code == xrecord.error_start {
                        out.push(RecordTarget::XRecordError);
                    }
                }
            }
            if target.client_started {
                out.push(RecordTarget::ClientStarted);
            }
            if target.client_died {
                out.push(RecordTarget::ClientDied);
            }
        }
        Ok(out)
    }

    pub(crate) fn process_targets(conn: &X11Connection, targets: impl IntoIterator<Item = RecordTarget>) -> Result<Vec<Range>> {
        let mut ranges: Vec<Range> = vec![];
        for target in targets {
            match target {
                RecordTarget::CoreRequest(opcode) => {
                    ranges.push(Range {
                        core_requests: Range8 {
                            first: opcode as u8,
                            last: opcode as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::CoreReply(opcode) => {
                    ranges.push(Range {
                        core_replies: Range8 {
                            first: opcode as u8,
                            last: opcode as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XgeRequest => {
                    let major_opcode = conn.get_ext_info(XGE_EXT_NAME).context("missing xge extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: 0,
                                last: 0,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XgeReply => {
                    let major_opcode = conn.get_ext_info(XGE_EXT_NAME).context("missing xge extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: 0,
                                last: 0,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::ShapeRequest(opcode) => {
                    let major_opcode = conn.get_ext_info(SHAPE_EXT_NAME).context("missing shape extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::ShapeReply(opcode) => {
                    let major_opcode = conn.get_ext_info(SHAPE_EXT_NAME).context("missing shape extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XFixesRequest(opcode) => {
                    let major_opcode = conn.get_ext_info(XFIXES_EXT_NAME).context("missing xfixes extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XFixesReply(opcode) => {
                    let major_opcode = conn.get_ext_info(XFIXES_EXT_NAME).context("missing xfixes extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XInputRequest(opcode) => {
                    let major_opcode = conn.get_ext_info(XINPUT_EXT_NAME).context("missing xinput extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XInputReply(opcode) => {
                    let major_opcode = conn.get_ext_info(XINPUT_EXT_NAME).context("missing xinput extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XKBRequest(opcode) => {
                    let major_opcode = conn.get_ext_info(XKB_EXT_NAME).context("missing XKB extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XKBReply(opcode) => {
                    let major_opcode = conn.get_ext_info(XKB_EXT_NAME).context("missing XKB extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XRandrRequest(opcode) => {
                    let major_opcode = conn.get_ext_info(XRANDR_EXT_NAME).context("missing xrandr extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XRandrReply(opcode) => {
                    let major_opcode = conn.get_ext_info(XRANDR_EXT_NAME).context("missing xrandr extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XRecordRequest(opcode) => {
                    let major_opcode = conn.get_ext_info(XRECORD_EXT_NAME).context("missing xrecord extension")?.major_opcode;
                    ranges.push(Range {
                        ext_requests: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XRecordReply(opcode) => {
                    let major_opcode = conn.get_ext_info(XRECORD_EXT_NAME).context("missing xrecord extension")?.major_opcode;
                    ranges.push(Range {
                        ext_replies: ExtRange {
                            major: Range8 {
                                first: major_opcode,
                                last: major_opcode,
                            },
                            minor: Range16 {
                                first: opcode as u8 as u16,
                                last: opcode as u8 as u16,
                            },
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeliveredCoreEvent(code) => {
                    ranges.push(Range {
                        delivered_events: Range8 {
                            first: code as u8,
                            last: code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeviceCoreEvent(code) => {
                    ranges.push(Range {
                        device_events: Range8 {
                            first: code as u8,
                            last: code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeliveredXkbEvent => {
                    let major_code = conn.get_ext_info(XKB_EXT_NAME).context("missing XKB extension")?.event_start;
                    ranges.push(Range {
                        delivered_events: Range8 {
                            first: major_code,
                            last: major_code,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeviceXkbEvent => {
                    let major_code = conn.get_ext_info(XKB_EXT_NAME).context("missing XKB extension")?.event_start;
                    ranges.push(Range {
                        device_events: Range8 {
                            first: major_code,
                            last: major_code,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeliveredXFixesEvent(code) => {
                    let major_code = conn.get_ext_info(XFIXES_EXT_NAME).context("missing xfixes extension")?.event_start;
                    ranges.push(Range {
                        delivered_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeviceXFixesEvent(code) => {
                    let major_code = conn.get_ext_info(XFIXES_EXT_NAME).context("missing xfixes extension")?.event_start;
                    ranges.push(Range {
                        device_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeliveredXRandrEvent(code) => {
                    let major_code = conn.get_ext_info(XRANDR_EXT_NAME).context("missing xrandr extension")?.event_start;
                    ranges.push(Range {
                        delivered_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeviceXRandrEvent(code) => {
                    let major_code = conn.get_ext_info(XRANDR_EXT_NAME).context("missing xrandr extension")?.event_start;
                    ranges.push(Range {
                        device_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeliveredShapeEvent(code) => {
                    let major_code = conn.get_ext_info(SHAPE_EXT_NAME).context("missing shape extension")?.event_start;
                    ranges.push(Range {
                        delivered_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeviceShapeEvent(code) => {
                    let major_code = conn.get_ext_info(SHAPE_EXT_NAME).context("missing shape extension")?.event_start;
                    ranges.push(Range {
                        device_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeliveredXInputEvent(code) => {
                    let major_code = conn.get_ext_info(XINPUT_EXT_NAME).context("missing xinput extension")?.event_start;
                    ranges.push(Range {
                        delivered_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::DeviceXInputEvent(code) => {
                    let major_code = conn.get_ext_info(XINPUT_EXT_NAME).context("missing xinput extension")?.event_start;
                    ranges.push(Range {
                        device_events: Range8 {
                            first: major_code + code as u8,
                            last: major_code + code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::CoreError(code) => {
                    ranges.push(Range {
                        errors: Range8 {
                            first: code as u8,
                            last: code as u8,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XkbError => {
                    let major_code = conn.get_ext_info(XKB_EXT_NAME).context("missing XKB extension")?.error_start;
                    ranges.push(Range {
                        errors: Range8 {
                            first: major_code,
                            last: major_code,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XInputError => {
                    let major_code = conn.get_ext_info(XINPUT_EXT_NAME).context("missing xinput extension")?.error_start;
                    ranges.push(Range {
                        errors: Range8 {
                            first: major_code,
                            last: major_code,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XFixesError => {
                    let major_code = conn.get_ext_info(XFIXES_EXT_NAME).context("missing xfixes extension")?.error_start;
                    ranges.push(Range {
                        errors: Range8 {
                            first: major_code,
                            last: major_code,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::XRecordError => {
                    let major_code = conn.get_ext_info(XRECORD_EXT_NAME).context("missing xrecord extension")?.error_start;
                    ranges.push(Range {
                        errors: Range8 {
                            first: major_code,
                            last: major_code,
                        },
                        ..Default::default()
                    });
                }
                RecordTarget::ClientStarted => {
                    ranges.push(Range {
                        client_started: true,
                        ..Default::default()
                    });
                }
                RecordTarget::ClientDied => {
                    ranges.push(Range {
                        client_died: true,
                        ..Default::default()
                    });
                }
            }
        }
        Ok(ranges)
    }
}
