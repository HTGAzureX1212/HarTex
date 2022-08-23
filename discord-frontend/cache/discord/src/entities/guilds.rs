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

use base::discord::model::guild::Guild;
use base::discord::model::id::{marker::GuildMarker, Id};
use cache_base::Entity;

pub struct CachedGuild {
    pub(crate) id: Id<GuildMarker>,
}

impl Entity for CachedGuild {
    type Id = Id<GuildMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<Guild> for CachedGuild {
    fn from(from: Guild) -> Self {
        Self { id: from.id }
    }
}

#[cfg(postgres)]
include!("postgres_backend_include/guilds.rs");
