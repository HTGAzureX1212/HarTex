/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hartex_core::dotenv;
use hartex_core::log;
use hartex_core::tokio;
use lapin::options::{ExchangeDeclareOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Connection, ConnectionProperties, ExchangeKind};

mod error;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> hartex_eyre::eyre::Result<()> {
    hartex_eyre::initialize()?;
    log::initialize();

    log::trace!("loading environment variables");
    dotenv::dotenv()?;

    let username = std::env::var("GATEWAY_RABBITMQ_USERNAME")?;
    let password = std::env::var("GATEWAY_RABBITMQ_PASSWORD")?;
    let host = std::env::var("RABBITMQ_HOST")?;
    let port = std::env::var("RABBITMQ_PORT")?;
    let uri = format!("amqp://{}:{}@{}:{}", username, password, host, port);
    let uri_log = format!("amqp://{}:<redacted>@{}:{}", username, host, port);

    log::trace!("creating rabbitmq amqp connection (uri: {})", &uri_log);
    let amqp_connection = Connection::connect(&uri, ConnectionProperties::default()).await?;

    let channel_recv = amqp_connection.create_channel().await?;
    let channel_send = amqp_connection.create_channel().await?;

    channel_recv
        .exchange_declare(
            "gateway",
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                passive: false,
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
            },
            FieldTable::default(),
        )
        .await?;

    channel_send
        .queue_declare(
            "gateway.send",
            QueueDeclareOptions {
                passive: false,
                durable: true,
                exclusive: false,
                auto_delete: false,
                nowait: false,
            },
            FieldTable::default(),
        ).await?;

    Ok(())
}
