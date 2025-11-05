use dashmap::DashMap;
use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::{
	requests::Requester,
	types::{ChatId, UserId},
	Bot,
};

use crate::{
	bot::{JoinRequest, JoinRequests},
	config::AppConfig,
	i18n::I18n, // AÑADIR
};

mod bot;
mod config;
mod server;
mod i18n; // AÑADIR

#[tokio::main]
async fn main() {
	dotenv().ok();
	pretty_env_logger::init();
	
	let config = AppConfig::try_read().expect("Failed to read config");
	let join_requests: JoinRequests = Arc::new(DashMap::<(ChatId, UserId), JoinRequest>::new());
	let i18n = Arc::new(I18n::new()); // AÑADIR
	let bot = Bot::new(&config.bot_token);
	let bot_data = bot.get_me().await.expect("Failed to get bot account");
	
	tokio::join!(
		bot::start(bot.clone(), config.clone(), join_requests.clone(), i18n.clone()), // MODIFICAR
		server::start(bot, config, bot_data.user, join_requests, i18n) // MODIFICAR
	);
}
