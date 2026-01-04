use clap::Parser;

use crate::{
    commands::Command,
    config::{
        Config,
        profile::{
            Profile,
            alias::ProfileAlias,
            keys::{Keys, auth::AuthKey, sign::SignKey},
            user::{User, email::UserEmail, name::UserName},
        },
    },
};

#[derive(Parser, Debug)]
pub struct UpsertOptions {
    /// The profile name (alias)
    #[arg(short, long)]
    pub alias: Option<String>,

    /// user.name config value
    #[arg(short, long)]
    pub name: Option<String>,

    /// user.email config value
    #[arg(short, long)]
    pub email: Option<String>,

    /// path to the ssh key to use for auth
    #[arg(short = 'k', long)]
    pub auth_key: Option<String>,

    /// user.signingkey config value
    #[arg(short, long)]
    pub sign_key: Option<String>,
}

impl Command for UpsertOptions {
    fn execute(&self, mut config: Config) {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config);

        let profile = config.profiles.get(&alias);

        let name = UserName::from_param(self.name.clone(), profile.map(|p| p.user.name.0.clone()));
        let email =
            UserEmail::from_param(self.email.clone(), profile.map(|p| p.user.email.0.clone()));

        let auth = AuthKey::from_param(
            self.auth_key.clone(),
            profile.map(|p| p.keys.auth.0.clone()),
        );
        let sign = SignKey::from_param(
            self.sign_key.clone(),
            profile.and_then(|p| p.keys.sign.clone().map(|k| k.0)),
        );

        config.profiles.insert(
            alias,
            Profile {
                user: User { name, email },
                keys: Keys { auth, sign },
            },
        );
        config.save().unwrap();
        println!("Profile saved successfully");
    }
}
