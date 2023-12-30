--! cached_guild_upsert (default_message_notifications, explicit_content_filter, features, icon?, large, name, owner_id, id, premium_subscription_count?, premium_tier)
INSERT INTO
    "DiscordFrontend"."Nightly"."CachedGuilds" ("default_message_notifications", "explicit_content_filter", "features", "icon", "large", "name", "owner_id", "id", "premium_subscription_count", "premium_tier")
VALUES (:default_message_notifications, :explicit_content_filter, :features, :icon, :large, :name, :owner_id, :id, :premium_subscription_count, :premium_tier)
ON CONFLICT ("id") DO UPDATE
    SET
        "default_message_notifications" = :default_message_notifications,
        "explicit_content_filter" = :explicit_content_filter,
        "features" = :features,
        "icon" = :icon,
        "large" = :large,
        "name" = :name,
        "owner_id" = :owner_id,
        "premium_subscription_count" = :premium_subscription_count,
        "premium_tier" = :premium_tier;
