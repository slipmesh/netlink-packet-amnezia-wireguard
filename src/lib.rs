// SPDX-License-Identifier: MIT

//! The `netlink-packet-amnezia-wireguard` crate is designed for parsing and
//! emitting generic netlink packets for Amnezia WireGuard interface.

pub mod range;

mod allowedip;
mod attribute;
mod message;
mod peer;
mod socket_addr;

// test data are using hard coded little endian byte order, not for big-endian
#[cfg(not(target_endian = "big"))]
#[cfg(test)]
mod test;

pub use self::{
    allowedip::{
        AmneziaWireguardAddressFamily, AmneziaWireguardAllowedIp,
        AmneziaWireguardAllowedIpAttr, AmneziaWireguardAllowedIpFlags,
    },
    attribute::{AmneziaWireguardAttribute, AmneziaWireguardDeviceFlags},
    message::{AmneziaWireguardCmd, AmneziaWireguardMessage},
    peer::{
        AmneziaWireguardPeer, AmneziaWireguardPeerAttribute,
        AmneziaWireguardPeerFlags, AmneziaWireguardTimeSpec,
    },
};
