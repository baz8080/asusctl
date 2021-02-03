//! # DBus interface proxy for: `org.asuslinux.Daemon`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/asuslinux/RogBios' from service 'org.asuslinux.Daemon' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://zeenix.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use std::sync::{Arc, Mutex};

use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.asuslinux.Daemon",
    default_path = "/org/asuslinux/RogBios"
)]
trait Daemon {
    /// DedicatedGraphicMode method
    fn dedicated_graphic_mode(&self) -> zbus::Result<i16>;

    /// PostBootSound method
    fn post_boot_sound(&self) -> zbus::Result<i16>;

    /// SetDedicatedGraphicMode method
    fn set_dedicated_graphic_mode(&self, dedicated: bool) -> zbus::Result<()>;

    /// SetPostBootSound method
    fn set_post_boot_sound(&self, on: bool) -> zbus::Result<()>;

    /// NotifyDedicatedGraphicMode signal
    #[dbus_proxy(signal)]
    fn notify_dedicated_graphic_mode(&self, dedicated: bool) -> zbus::Result<()>;

    /// NotifyPostBootSound signal
    #[dbus_proxy(signal)]
    fn notify_post_boot_sound(&self, dedicated: bool) -> zbus::Result<()>;
}

pub struct RogBiosProxy<'a>(DaemonProxy<'a>);

impl<'a> RogBiosProxy<'a> {
    #[inline]
    pub fn new(conn: &Connection) -> Result<Self> {
        Ok(RogBiosProxy(DaemonProxy::new(&conn)?))
    }

    pub fn proxy(&self) -> &DaemonProxy<'a> {
        &self.0
    }

    #[inline]
    pub fn get_dedicated_gfx(&self) -> Result<i16> {
        self.0.dedicated_graphic_mode()
    }

    #[inline]
    pub fn set_dedicated_gfx(&self, on: bool) -> Result<()> {
        self.0.set_dedicated_graphic_mode(on)
    }

    #[inline]
    pub fn get_post_sound(&self) -> Result<i16> {
        self.0.post_boot_sound()
    }

    #[inline]
    pub fn set_post_sound(&self, on: bool) -> Result<()> {
        self.0.set_post_boot_sound(on)
    }

    #[inline]
    pub fn connect_notify_dedicated_graphic_mode(
        &self,
        dedicated: Arc<Mutex<Option<bool>>>,
    ) -> zbus::fdo::Result<()> {
        self.0.connect_notify_dedicated_graphic_mode(move |data| {
            if let Ok(mut lock) = dedicated.lock() {
                *lock = Some(data);
            }
            Ok(())
        })
    }

    #[inline]
    pub fn connect_notify_post_boot_sound(
        &self,
        sound: Arc<Mutex<Option<bool>>>,
    ) -> zbus::fdo::Result<()> {
        self.0.connect_notify_post_boot_sound(move |data| {
            if let Ok(mut lock) = sound.lock() {
                *lock = Some(data);
            }
            Ok(())
        })
    }
}
