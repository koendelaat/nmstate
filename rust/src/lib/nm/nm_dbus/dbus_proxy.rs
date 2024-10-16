// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    #[dbus_proxy(property)]
    fn version(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn active_connections(
        &self,
    ) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn checkpoints(&self) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;

    #[dbus_proxy(property)]
    fn global_dns_configuration(
        &self,
    ) -> zbus::Result<HashMap<String, zvariant::OwnedValue>>;

    /// CheckpointCreate method
    fn checkpoint_create(
        &self,
        devices: &[zvariant::ObjectPath],
        rollback_timeout: u32,
        flags: u32,
    ) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// CheckpointDestroy method
    fn checkpoint_destroy(
        &self,
        checkpoint: &zvariant::ObjectPath,
    ) -> zbus::Result<()>;

    /// CheckpointRollback method
    fn checkpoint_rollback(
        &self,
        checkpoint: &zvariant::ObjectPath,
    ) -> zbus::Result<HashMap<String, u32>>;

    /// ActivateConnection method
    fn activate_connection(
        &self,
        connection: &zvariant::ObjectPath,
        device: &zvariant::ObjectPath,
        specific_object: &zvariant::ObjectPath,
    ) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// DeactivateConnection method
    fn deactivate_connection(
        &self,
        active_connection: &zvariant::ObjectPath,
    ) -> zbus::Result<()>;

    /// GetAllDevices method
    fn get_all_devices(&self) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;

    /// CheckpointAdjustRollbackTimeout method
    fn checkpoint_adjust_rollback_timeout(
        &self,
        checkpoint: &zvariant::ObjectPath,
        add_timeout: u32,
    ) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Settings",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
trait NetworkManagerSetting {
    /// GetConnectionByUuid method
    fn get_connection_by_uuid(
        &self,
        uuid: &str,
    ) -> zbus::Result<zvariant::OwnedObjectPath>;

    /// AddConnection2 method
    fn add_connection2(
        &self,
        settings: HashMap<&str, HashMap<&str, zvariant::Value>>,
        flags: u32,
        args: HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<(
        zvariant::OwnedObjectPath,
        HashMap<String, zvariant::OwnedValue>,
    )>;

    /// ListConnections method
    fn list_connections(&self) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;

    /// GetAllDevices method
    fn get_all_devices(&self) -> zbus::Result<Vec<zvariant::OwnedObjectPath>>;

    /// SaveHostname method
    fn save_hostname(&self, hostname: &str) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.DnsManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/DnsManager"
)]
trait NetworkManagerDns {
    /// Configuration property
    #[dbus_proxy(property)]
    fn configuration(
        &self,
    ) -> zbus::Result<Vec<HashMap<String, zvariant::OwnedValue>>>;
}
