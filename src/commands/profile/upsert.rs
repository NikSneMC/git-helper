use clap::Parser;

use crate::{
    commands::Command,
    config::{
        Config,
        profile::{
            Profile,
            alias::ProfileAlias,
            credential::{Credential, username::CredentialUsername},
            user::{User, email::UserEmail, name::UserName, signingkey::UserSigningKey},
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

    /// user.signingkey config value
    #[arg(short, long)]
    pub signingkey: Option<String>,

    /// credential.username config value
    #[arg(short, long)]
    pub username: Option<String>,
}

impl Command for UpsertOptions {
    fn execute(&self, mut config: Config) {
        let alias = ProfileAlias::from_param(self.alias.clone(), &config, true);

        let profile = config.profiles.get(&alias);

        let name = UserName::from_param(self.name.clone(), profile.map(|p| p.user.name.0.clone()));
        let email =
            UserEmail::from_param(self.email.clone(), profile.map(|p| p.user.email.0.clone()));
        let signing_key = UserSigningKey::from_param(
            self.signingkey.clone(),
            profile.and_then(|p| p.user.signingkey.clone().map(|k| k.0)),
        );

        let username = CredentialUsername::from_param(
            self.username.clone(),
            profile.and_then(|p| p.credential.username.clone().map(|u| u.0)),
        );

        config.profiles.insert(
            alias,
            Profile {
                user: User {
                    name,
                    email,
                    signingkey: signing_key,
                },
                credential: Credential { username },
            },
        );
        config.save().unwrap();
        println!("Profile saved successfully");
    }
}
