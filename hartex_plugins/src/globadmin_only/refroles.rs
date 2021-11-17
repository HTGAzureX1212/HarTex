//! # The `refroles` Module
//!
//! This module implements the `refroles` command.

use hartex_cmdsys::{
    command::{
        Command,
        CommandType
    },
    context::CommandContext
};
use hartex_core::{
    discord::{
        cache_inmemory::CloneableInMemoryCache,
        model::application::{
            callback::{
                CallbackData,
                InteractionResponse
            },
            interaction::Interaction
        }
    },
    error::{
        HarTexError,
        HarTexResult
    },
    logging::tracing
};
use hartex_dbmani::{
    guildconf::GetGuildConfig,
    whitelist::GetWhitelistedGuilds
};
use hartex_utils::FutureRetType;
use tokio::time;

use crate::PLUGIN_ENV;

/// # Struct `Refroles`
///
/// The `refroles` command.
pub struct Refroles;

impl Command for Refroles {
    fn name(&self) -> String {
        String::from("refroles")
    }

    fn description(&self) -> String {
        String::from("GlobAdminOnlyPlugin.RefrolesCommand")
    }

    fn command_type(&self) -> CommandType {
        CommandType::ChatInput
    }

    fn execute<'asynchronous_trait>(
        &self,
        ctx: CommandContext,
        cache: CloneableInMemoryCache
    ) -> FutureRetType<'asynchronous_trait, ()> {
        Box::pin(execute_refroles_command(ctx, cache))
    }
}

/// # Asynchronous Function `execute_refroles_command`
///
/// Executes the `refroles` command.
///
/// ## Parameters
/// - `ctx`, type `CommandContext`: the command context to use.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::too_many_lines)]
async fn execute_refroles_command(
    ctx: CommandContext,
    cache: CloneableInMemoryCache
) -> HarTexResult<()> {
    let interaction = if let Interaction::ApplicationCommand(command) = ctx.interaction.clone() {
        command
    }
    else {
        tracing::error!("invalid interaction type: expected ApplicationCommand");

        return Err(HarTexError::Custom {
            message: String::from("invalid interaction type: expected ApplicationCommand")
        });
    };

    if interaction.guild_id.is_none() || interaction.user.is_some() {
        ctx.http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(String::from(
                        ":x: This command can only be used in a guild."
                    )),
                    embeds: vec![],
                    flags: None,
                    tts: None
                })
            )
            .exec()
            .await?;
    }

    if interaction.member.unwrap().user.unwrap().id != PLUGIN_ENV.global_administrator_uid.unwrap()
    {
        ctx.http
            .interaction_callback(
                interaction.id,
                &interaction.token,
                &InteractionResponse::ChannelMessageWithSource(CallbackData {
                    allowed_mentions: None,
                    components: None,
                    content: Some(String::from(":x: You are not the global administrator.")),
                    embeds: vec![],
                    flags: None,
                    tts: None
                })
            )
            .exec()
            .await?;
    }

    let guilds = GetWhitelistedGuilds::default().await?;
    let owners = guilds
        .iter()
        .map(|guild| cache.guild(guild.GuildId).unwrap().owner_id())
        .collect::<Vec<_>>();
    let owner_role_members = cache
        .guild_members(PLUGIN_ENV.support_guild_gid.unwrap())
        .unwrap();
    let to_remove = owner_role_members
        .clone()
        .into_iter()
        .filter(|uid| !owners.contains(uid))
        .collect::<Vec<_>>();

    // FIXME: add access role removal
    for guild in guilds {
        tracing::trace!("current guild: {id}", id = guild.GuildId);
        let config = GetGuildConfig::new(guild.GuildId).await?;

        for access in config.DashboardAccess {
            if let Err(error) = ctx
                .http
                .add_guild_member_role(
                    PLUGIN_ENV.support_guild_gid.unwrap(),
                    access.userId,
                    PLUGIN_ENV.hartex_user_rid.unwrap()
                )
                .exec()
                .await {
                tracing::trace!("failed to add hartex user role to member: {error:?}");
            }

            time::sleep(time::Duration::from_secs(1)).await;
        }
    }

    for uid in to_remove {
        if let Err(error) = ctx
            .http
            .remove_guild_member_role(
                PLUGIN_ENV.support_guild_gid.unwrap(),
                uid,
                PLUGIN_ENV.hartex_guild_owner_rid.unwrap()
            )
            .exec()
            .await {
            tracing::error!("failed to remove hartex guild owner role from member: {error:?}");
        }

        time::sleep(time::Duration::from_secs(1)).await;
    }

    for owner in owners {
        if let Err(error) = ctx
            .http
            .add_guild_member_role(
                PLUGIN_ENV.support_guild_gid.unwrap(),
                owner,
                PLUGIN_ENV.hartex_guild_owner_rid.unwrap()
            )
            .exec()
            .await {
            tracing::error!("failed to add hartex guild owner role to member: {error:?}");
        }

        time::sleep(time::Duration::from_secs(1)).await;
    }

    ctx.http
        .interaction_callback(
            interaction.id,
            &interaction.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: Some(String::from("Done")),
                embeds: vec![],
                flags: None,
                tts: None
            })
        )
        .exec()
        .await?;

    Ok(())
}
