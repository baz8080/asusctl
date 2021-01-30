// This code was autogenerated with `dbus-codegen-rust -s -d org.asuslinux.Daemon -f org.asuslinux.Daemon -c blocking -p /org/asuslinux/Supported -m None -o asus-nb/src/dbus_supported.rs`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgAsuslinuxDaemon {
    fn supported_functions(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgAsuslinuxDaemon
    for blocking::Proxy<'a, C>
{
    fn supported_functions(&self) -> Result<String, dbus::Error> {
        self.method_call("org.asuslinux.Daemon", "SupportedFunctions", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}