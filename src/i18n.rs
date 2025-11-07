use serde::Deserialize;
use std::collections::HashMap;
use teloxide::types::User;

#[derive(Debug, Clone, Deserialize)]
pub struct Translation {
    pub new_user_template: String,
    pub unauthorized_group: String,
    pub successfully_verified: String,
    pub user_doesnt_match_error: String,
    pub verify_button: String,
    pub help_private: String,
    pub help_group_check_ok: String,
    pub help_group_check_fail: String,
    pub help_use_in_group: String,
    pub alert_success: String,
    pub alert_already_used: String,
    pub alert_error: String,
}

impl Default for Translation {
    fn default() -> Self {
        Self {
            new_user_template: "👋 gm {TAGUSER}! Welcome to {CHATNAME}.\nTo access the group, please verify your account with World ID.".to_string(),
            user_doesnt_match_error: "❌ This message isn't for you".to_string(),
            successfully_verified: "✅ Verified with World ID. Welcome to the group!".to_string(),
            unauthorized_group: "❌ You can't use this bot on this group. Bye!".to_string(),
            verify_button: "Verify with World ID".to_string(),
            help_private: "Welcome to the World ID Telegram bot!\nYou can use me to protect your group from spammers and bots. To get started, add me to your (public) group and give me admin permissions. When someone joins your group, they'll be asked to prove they're human with World ID before they can send messages.".to_string(),
            help_group_check_ok: "Bot has admin permissions and is ready to go! Once someone joins the group, they'll be asked to prove they're human with World ID before they can send messages.".to_string(),
            help_group_check_fail: "Bot doesn't have admin permissions! Please, give it admin permissions and try again.".to_string(),
            help_use_in_group: "You can only use this bot in public groups. Please add me to a public group (with admin permissions) and try again.".to_string(),
            alert_success: "Successfully verified! You can now close this and go back to the group.".to_string(),
            alert_already_used: "This World ID has already been used to join this group. You can't do it again!".to_string(),
            alert_error: "Something went wrong, please try again later.".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct I18n {
    translations: HashMap<String, Translation>,
    default_lang: String,
}

impl I18n {
    pub fn new() -> Self {
        let mut translations = HashMap::new();
        
        // English
        translations.insert("en".to_string(), Translation::default());
        
        // Spanish
        translations.insert("es".to_string(), Translation {
            new_user_template: "👋 ¡Hola {TAGUSER}! Bienvenido a {CHATNAME}.\nPara acceder al grupo, por favor verifica tu cuenta con World ID.".to_string(),
            user_doesnt_match_error: "❌ Este mensaje no es para ti".to_string(),
            successfully_verified: "✅ Verificado con World ID. ¡Bienvenido al grupo!".to_string(),
            unauthorized_group: "❌ No puedes usar este bot en este grupo. ¡Adiós!".to_string(),
            verify_button: "Verificar con World ID".to_string(),
            help_private: "¡Bienvenido al bot de Telegram de World ID!\nPuedes usarme para proteger tu grupo de spammers y bots. Para comenzar, añádeme a tu grupo (público) y dame permisos de administrador. Cuando alguien se una a tu grupo, se le pedirá que demuestre que es humano con World ID antes de poder enviar mensajes.".to_string(),
            help_group_check_ok: "¡El bot tiene permisos de administrador y está listo! Una vez que alguien se una al grupo, se le pedirá que demuestre que es humano con World ID antes de poder enviar mensajes.".to_string(),
            help_group_check_fail: "¡El bot no tiene permisos de administrador! Por favor, dale permisos de administrador e intenta nuevamente.".to_string(),
            help_use_in_group: "Solo puedes usar este bot en grupos públicos. Por favor añádeme a un grupo público (con permisos de administrador) e intenta nuevamente.".to_string(),
            alert_success: "¡Verificación exitosa! Ahora puedes cerrar esto y volver al grupo.".to_string(),
            alert_already_used: "Este World ID ya ha sido usado para unirse a este grupo. ¡No puedes hacerlo de nuevo!".to_string(),
            alert_error: "Algo salió mal, por favor intenta nuevamente más tarde.".to_string(),
        });
        
        // Portuguese
        translations.insert("pt".to_string(), Translation {
            new_user_template: "👋 Olá {TAGUSER}! Bem-vindo ao {CHATNAME}.\nPara acessar o grupo, por favor verifique sua conta com World ID.".to_string(),
            user_doesnt_match_error: "❌ Esta mensagem não é para você".to_string(),
            successfully_verified: "✅ Verificado com World ID. Bem-vindo ao grupo!".to_string(),
            unauthorized_group: "❌ Você não pode usar este bot neste grupo. Tchau!".to_string(),
            verify_button: "Verificar com World ID".to_string(),
            help_private: "Bem-vindo ao bot do Telegram World ID!\nVocê pode me usar para proteger seu grupo de spammers e bots. Para começar, adicione-me ao seu grupo (público) e me dê permissões de administrador. Quando alguém entrar no seu grupo, será solicitado que prove que é humano com World ID antes de poder enviar mensagens.".to_string(),
            help_group_check_ok: "O bot tem permissões de administrador e está pronto! Uma vez que alguém entre no grupo, será solicitado que prove que é humano com World ID antes de poder enviar mensagens.".to_string(),
            help_group_check_fail: "O bot não tem permissões de administrador! Por favor, dê permissões de administrador e tente novamente.".to_string(),
            help_use_in_group: "Você só pode usar este bot em grupos públicos. Por favor, adicione-me a um grupo público (com permissões de administrador) e tente novamente.".to_string(),
            alert_success: "Verificação bem-sucedida! Agora você pode fechar isso e voltar ao grupo.".to_string(),
            alert_already_used: "Este World ID já foi usado para entrar neste grupo. Você não pode fazer isso novamente!".to_string(),
            alert_error: "Algo deu errado, por favor tente novamente mais tarde.".to_string(),
        });
        
        Self {
            translations,
            default_lang: "en".to_string(),
        }
    }
    
    pub fn get(&self, lang: &str) -> &Translation {
        self.translations
            .get(lang)
            .unwrap_or_else(|| self.translations.get(&self.default_lang).unwrap())
    }

    pub fn detect_language<'a>(&'a self, user: Option<&'a User>) -> &'a str {
    if let Some(user) = user {
        if let Some(lang_code) = &user.language_code {
            let lang = lang_code.split('-').next().unwrap_or("en");
            if self.translations.contains_key(lang) {
                return lang;
            }
        }
    }
    self.default_lang.as_str()
}

    
    pub fn available_languages(&self) -> Vec<String> {
        self.translations.keys().cloned().collect()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}
