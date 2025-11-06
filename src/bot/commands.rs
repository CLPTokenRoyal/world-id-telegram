use std::sync::Arc;
use teloxide::{
	payloads::SendMessageSetters,
	requests::Requester,
	types::{Me, Message},
	utils::command::BotCommands,
	Bot,
};

use crate::{
	bot::HandlerResult,
	config::{AppConfig, GroupsConfig},
	i18n::I18n, // AÑADIR
};

#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
	#[command(description = "Explain how this bot works, and how to use it.")]
	Help,
	#[command(description = "Check that the bot is online and has the right permissions.")]
	Check,
	#[command(description = "Initial help when talking to the bot for the first time.")]
	Start,
}

pub async fn command_handler(
	bot: Bot,
	config: Arc<AppConfig>,
	msg: Message,
	me: Me,
	text: String,
	i18n: Arc<I18n>, // AÑADIR ESTE PARÁMETRO
) -> HandlerResult {
	if msg.from().is_none() {
		return Ok(());
	}
	
	if !config.groups_config.is_group_allowed(msg.chat.id) {
		return on_group_not_allowed(bot, &config.groups_config, msg, i18n).await; // MODIFICAR
	}
	
	let Ok(command) = BotCommands::parse(text.as_str(), me.username()) else {
		return Ok(());
	};
	
	// Detectar idioma
	let group_settings = config.groups_config.get(msg.chat.id);
	let lang = group_settings
		.language
		.as_ref()
		.map(|s| s.as_str())
		.unwrap_or_else(|| i18n.detect_language(msg.from()));
	let translation = i18n.get(lang);
	
	match command {
		Command::Check => {
			if msg.chat.is_private() {
				bot.send_message(msg.chat.id, &translation.help_use_in_group)
					.reply_to_message_id(msg.id)
					.await?;
				return Ok(());
			}
			
			let is_admin = bot
				.get_chat_member(msg.chat.id, bot.get_me().await?.id)
				.await?
				.is_administrator();
			
			let message = if is_admin {
				&translation.help_group_check_ok
			} else {
				&translation.help_group_check_fail
			};
			
			bot.send_message(msg.chat.id, message)
				.reply_to_message_id(msg.id)
				.await?;
		},
		Command::Help | Command::Start => {
			if msg.chat.is_private() {
				bot.send_message(msg.chat.id, &translation.help_private)
					.reply_to_message_id(msg.id)
					.await?;
			}
		},
	};
	
	Ok(())
}

pub async fn on_group_not_allowed(
	bot: Bot,
	config: &GroupsConfig,
	msg: Message,
	i18n: Arc<I18n>, // AÑADIR ESTE PARÁMETRO
) -> HandlerResult {
	log::error!(
		"Unknown chat {} with id {}",
		msg.chat.title().unwrap_or_default(),
		msg.chat.id
	);
	
	// Detectar idioma del usuario
	let lang = i18n.detect_language(msg.from());
	let translation = i18n.get(&lang);
	
	bot.send_message(msg.chat.id, &translation.unauthorized_group)
		.await?;
	bot.leave_chat(msg.chat.id).await?;
	
	Ok(())
}
