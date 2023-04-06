/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hartex_discord_commands_core::CommandMetadata;
use hartex_discord_commands_core::traits::Command;
use hartex_discord_core::discord::model::application::interaction::Interaction;
use hartex_discord_core::discord::util::builder::embed::EmbedAuthorBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedBuilder;
use hartex_discord_core::discord::util::builder::embed::EmbedFooterBuilder;
use hartex_discord_utils::CLIENT;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;

#[derive(CommandMetadata)]
#[metadata(command_type = 1)]
#[metadata(interaction_only = true)]
#[metadata(name = "about")]
pub struct Contributors;

impl Command for Contributors {
    async fn execute(&self, interaction: Interaction) -> hartex_eyre::Result<()> {
        let _ = CLIENT.interaction(interaction.application_id);
        let bundle = create_bundle(
            interaction.locale.and_then(|locale| locale.parse().ok()),
            &["discord-frontend", "commands"],
        )?;

        bundle_get!(bundle."contributors-embed-title": message, out [contributors_embed_title, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-description": message, out [contributors_embed_description, errors]);
        handle_errors(errors)?;
        bundle_get!(bundle."contributors-embed-footer": message, out [contributors_embed_footer, errors]);
        handle_errors(errors)?;

        let _ = EmbedBuilder::new()
            .author(EmbedAuthorBuilder::new(contributors_embed_title).build())
            .description(contributors_embed_description)
            .footer(EmbedFooterBuilder::new(contributors_embed_footer))
            .build();

        todo!()
    }
}
