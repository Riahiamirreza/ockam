mod create;
mod default;
mod delete;
mod list;
mod show;

use anyhow::anyhow;
use colorful::Colorful;
pub(crate) use create::CreateCommand;
pub(crate) use delete::DeleteCommand;
pub(crate) use list::ListCommand;
pub(crate) use show::ShowCommand;

use crate::identity::default::DefaultCommand;
use crate::terminal::OckamColor;
use crate::{docs, fmt_log, fmt_ok, CommandGlobalOpts, Result, PARSER_LOGS};
use clap::{Args, Subcommand};
use ockam_api::cli_state::traits::StateDirTrait;

const LONG_ABOUT: &str = include_str!("./static/long_about.txt");

/// Manage identities
#[derive(Clone, Debug, Args)]
#[command(
arg_required_else_help = true,
subcommand_required = true,
long_about = docs::about(LONG_ABOUT),
)]
pub struct IdentityCommand {
    #[command(subcommand)]
    subcommand: IdentitySubcommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum IdentitySubcommand {
    Create(CreateCommand),
    Show(ShowCommand),
    List(ListCommand),
    Default(DefaultCommand),
    Delete(DeleteCommand),
}

impl IdentityCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        match self.subcommand {
            IdentitySubcommand::Create(c) => c.run(options),
            IdentitySubcommand::Show(c) => c.run(options),
            IdentitySubcommand::List(c) => c.run(options),
            IdentitySubcommand::Delete(c) => c.run(options),
            IdentitySubcommand::Default(c) => c.run(options),
        }
    }
}

// If a name is given try to parse it as an identity name, otherwise return the default identity name
pub fn get_identity_name(opts: &CommandGlobalOpts, name: Option<String>) -> Result<String> {
    match name {
        Some(n) => identity_name_parser(opts, &n),
        None => {
            let name = default_identity_name(opts)?;
            identity_name_parser(opts, &name)
        }
    }
}

pub fn identity_name_parser(opts: &CommandGlobalOpts, identity_name: &str) -> Result<String> {
    if identity_name == "default" && opts.state.identities.default().is_err() {
        return Ok(create_default_identity(opts, identity_name));
    }

    Ok(identity_name.to_string())
}

pub fn default_identity_name(opts: &CommandGlobalOpts) -> Result<String> {
    match opts.state.identities.default().ok() {
        Some(i) => Ok(i.name().to_string()),
        None => {
            Err(anyhow!("Default identity not found. Have you run 'ockam identity create'?").into())
        }
    }
}

pub fn create_default_identity(opts: &CommandGlobalOpts, identity_name: &str) -> String {
    let create_command = CreateCommand::new(identity_name.into(), None);
    create_command.run(opts.clone());

    if let Ok(mut logs) = PARSER_LOGS.lock() {
        logs.push(fmt_log!("No default identity was found."));
        logs.push(fmt_ok!(
            "Creating default identity {}",
            identity_name
                .to_string()
                .color(OckamColor::PrimaryResource.color())
        ));
        logs.push(fmt_log!(
            "Setting identity {} as default for local operations...\n",
            identity_name
                .to_string()
                .color(OckamColor::PrimaryResource.color())
        ));
    }

    identity_name.to_string()
}
