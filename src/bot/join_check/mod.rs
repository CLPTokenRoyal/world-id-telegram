use std::sync::Arc;
use tokio::time::{sleep, Duration};
use teloxide::{
    prelude::*,
    types::{ChatPermissions, InlineKeyboardButton, InlineKeyboardMarkup, User},
    utils::html::escape,
};

use crate::{
    bot::{commands::on_group_not_allowed, HandlerResult, JoinRequest, JoinRequests},
    config::{AppConfig, MessagesText},
    i18n::I18n,
};

/// Maneja la llegada de nuevos usuarios al grupo
pub async fn join_handler(
    bot: Bot,
    msg: Message,
    users: Vec<User>,
    config: Arc<AppConfig>,
    join_requests: JoinRequests,
    i18n: Arc<I18n>,
) -> HandlerResult {
    // Si el grupo no está permitido, no continuamos
    if !config.groups_config.is_group_allowed(msg.chat.id) {
        return on_group_not_allowed(bot, &config.groups_config, msg, i18n).await;
    }

    let chat_cfg = config.groups_config.get(msg.chat.id);

    for user in users {
        if user.is_bot {
            continue;
        }

        let join_requests = join_requests.clone();

        // Detectar idioma basado en configuración o detección automática
        let lang = chat_cfg
            .language
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or_else(|| i18n.detect_language(Some(&user)));

        let translation = i18n.get(lang);
        let messages = MessagesText::from_translation(translation);

        // Crear mensaje de bienvenida traducido
        let chat_name = escape(if let Some(ref title) = chat_cfg.chat_name {
            title
        } else {
            msg.chat.title().unwrap_or_default()
        });

        let welcome_msg = messages.create_welcome_msg(&user, &chat_name);

        // Restringir permisos inicialmente (no enviar mensajes, etc.)
        bot.restrict_chat_member(msg.chat.id, user.id, ChatPermissions::empty())
            .await?;

        // Botón de verificación World ID
        let verify_button = InlineKeyboardButton::url(
            &translation.verify_button,
            config
                .app_url
                .join(&format!("verify/{}/{}", msg.chat.id, user.id))?,
        );

        // Enviar mensaje de bienvenida con botón
        let msg_id = bot
            .send_message(msg.chat.id, welcome_msg)
            .reply_to_message_id(msg.id)
            .parse_mode(teloxide::types::ParseMode::Html)
            .reply_markup(InlineKeyboardMarkup::new([vec![verify_button]]))
            .await?
            .id;

        // Registrar la solicitud de unión pendiente
        join_requests.insert((msg.chat.id, user.id), JoinRequest::new(msg_id));

        // Lanzar tarea asíncrona para verificar si el usuario se valida
        tokio::spawn({
            let bot = bot.clone();
            let join_requests = join_requests.clone();
            let user = user.clone();
            let translation = translation.clone();
            let ban_after = chat_cfg.ban_after;

            async move {
                // Esperar el tiempo configurado antes de expulsar
                sleep(ban_after).await;

                if let Some((_, data)) = join_requests.remove(&(msg.chat.id, user.id)) {
                    if !data.is_verified {
                        // 1️⃣ Expulsar al usuario del grupo
                        if let Err(err) = bot.kick_chat_member(msg.chat.id, user.id).await {
                            log::error!("Error al expulsar a {}: {}", user.full_name(), err);
                        } else {
                            log::info!("Usuario {} expulsado por no verificarse", user.full_name());
                        }

                        // 2️⃣ Eliminar el mensaje de verificación del grupo
                        if let Some(msg_id) = data.msg_id {
                            if let Err(err) = bot.delete_message(msg.chat.id, msg_id).await {
                                log::warn!("No se pudo eliminar el mensaje de verificación: {}", err);
                            }
                        }

                        // 3️⃣ Intentar enviar mensaje privado
                        if let Err(err) = bot
                            .send_message(user.id, &translation.user_doesnt_match_error)
                            .await
                        {
                            log::warn!(
                                "No se pudo enviar mensaje privado a {}: {}",
                                user.full_name(),
                                err
                            );
                        }
                    }
                }
            }
        });
    }

    Ok(())
}

/// Marca al usuario como verificado (lógica de World ID)
pub async fn on_verified(
    bot: Bot,
    chat_id: ChatId,
    user_id: UserId,
    join_requests: JoinRequests,
) -> HandlerResult {
    let mut join_req = join_requests
        .get_mut(&(chat_id, user_id))
        .ok_or("Can't find the message id in group dialogue")?;

    let Some(permissions) = bot.get_chat(chat_id).await?.permissions() else {
        return Err("Can't get the group permissions".into());
    };

    // Marcar como verificado
    join_req.is_verified = true;

    // Restaurar permisos originales
    bot.restrict_chat_member(chat_id, user_id, permissions)
        .await?;

    // Eliminar mensaje de bienvenida
    if let Some(msg_id) = join_req.msg_id.take() {
        bot.delete_message(chat_id, msg_id).await?;
    }

    Ok(())
}
