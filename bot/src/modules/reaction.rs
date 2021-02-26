use serenity::{
    client::Context,
    model::{
        channel::{Reaction, ReactionType},
        guild::{Guild, Member, PartialGuild, Role},
        id::RoleId,
    },
};

pub async fn handle_reaction_event(context: Context, reaction: Reaction) {
    debug!("Detected reaction add, started processing...");
    if let Some(g_id) = reaction.guild_id {
        if let Some(u_id) = reaction.user_id {
            debug!("Fetching guild...");
            if let Ok(g) = Guild::get(context.http.clone(), g_id).await {
                debug!("Fetching user...");
                if let Ok(m) = g.member(context.http.clone(), u_id).await {
                    debug!("Successfully fetched required data, started add reaction handler...");
                    handle_reaction(context.clone(), m, g, reaction.emoji).await;
                }
            }
        }
    }
}

async fn handle_reaction(ctx: Context, member: Member, guild: PartialGuild, _emoji: ReactionType) {
    // TODO: Integrate with database.
    debug!("Fetching role...");
    if let Some(role) = guild.clone().roles.get(&RoleId(814931174349602887)) {
        debug!("Successfully fetched the role, started role toggling");
        toggle_role(ctx, role, member).await;
    }
}

async fn toggle_role(ctx: Context, role: &Role, member: Member) {
    if member.roles.contains(&role.id) {
        debug!("Member has role, removing role...");
        if let Err(e) = member.clone().remove_role(ctx.http, role).await {
            error!("Could not auto remove role from member!\r\n{}", e);
        }
    } else {
        debug!("Member does not have role, giving role...");
        if let Err(e) = member.clone().add_role(ctx.http, role).await {
            error!("Could not auto remove role from member!\r\n{}", e);
        }
    }
    debug!("Toggled the role for the member!");
}

