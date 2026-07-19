use std::collections::HashMap;

use zbus::Connection;

use zbus::zvariant::Value;

use crate::Result;

#[derive(Debug, Clone)]
pub struct Notifier {
    connection: Connection,
}

impl Notifier {
    pub async fn new() -> Result<Self> {
        let connection = Connection::session().await?;

        Ok(Self { connection })
    }

    pub async fn notify(&self, summary: &str, body: &str) -> Result<()> {
        let proxy = zbus::Proxy::new(
            &self.connection,
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
        )
        .await?;

        let hints: HashMap<&str, Value<'_>> = HashMap::new();

        let _: u32 = proxy
            .call(
                "Notify",
                &(
                    "hypr-download-sorter",
                    0u32,
                    "",
                    summary,
                    body,
                    Vec::<String>::new(),
                    hints,
                    5000i32,
                ),
            )
            .await?;

        Ok(())
    }
}
