use teloxide::{
    prelude::*,
    types::{Chat, ChatId, ChatKind, Message},
};

use crate::config::AppConfig;

/// Envía un aviso y abandona grupos no autorizados.
/// Debe llamarse cuando el bot es agregado a un grupo que no está en la lista blanca.
pub async fn on_group_not_allowed(bot: Bot, msg: Message, app_config: AppConfig) -> anyhow::Result<()> {
    let chat = match &msg.chat {
        Chat {
            kind: ChatKind::Public(_) | ChatKind::Supergroup(_),
            id,
            title,
            ..
        } => (*id, title.clone()),
        _ => return Ok(()), // Ignorar mensajes fuera de grupos
    };

    let chat_id = ChatId(chat.0);

    // Si el grupo está permitido, no hacer nada
    if app_config.groups_config.is_group_allowed(chat_id) {
        return Ok(());
    }

    let title_display = chat.1.unwrap_or_else(|| "this group".to_string());
    let text = format!(
        "❌ This bot is not authorized to operate in <b>{}</b>.\n\nIf you want to use it, please contact the bot admin.",
        title_display
    );

    bot.send_message(chat_id, text)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await
        .ok();

    // Esperar unos segundos antes de salir (permite ver el mensaje)
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    bot.leave_chat(chat_id).await.ok();

    Ok(())
}
