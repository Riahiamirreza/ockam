use crate::run::parser::resources::{parse_cmd_from_args, ArgsToCommands, ResourceNameOrMap};
use crate::tcp::inlet::create::CreateCommand;
use crate::{tcp::inlet, OckamSubcommand};
use miette::{miette, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TcpInlets {
    #[serde(alias = "tcp-inlets")]
    pub tcp_inlets: Option<ResourceNameOrMap>,
}

impl ArgsToCommands<CreateCommand> for TcpInlets {
    fn into_commands(self) -> Result<Vec<CreateCommand>> {
        let get_subcommand = |args: &[String]| -> Result<CreateCommand> {
            if let OckamSubcommand::TcpInlet(cmd) = parse_cmd_from_args("tcp-inlet create", args)? {
                if let inlet::TcpInletSubCommand::Create(c) = cmd.subcommand {
                    return Ok(c);
                }
            }
            Err(miette!("Failed to parse command"))
        };
        match self.tcp_inlets {
            Some(c) => c.into_commands(get_subcommand, Some("alias")),
            None => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::str::FromStr;

    #[test]
    fn tcp_inlet_config() {
        let named = r#"
            tcp_inlets:
              ti1:
                from: 6060
                at: n
              ti2:
                from: '6061'
                alias: my_inlet
        "#;
        let parsed: TcpInlets = serde_yaml::from_str(named).unwrap();
        let cmds = parsed.into_commands().unwrap();
        assert_eq!(cmds.len(), 2);
        assert_eq!(cmds[0].alias, "ti1");
        assert_eq!(
            cmds[0].from,
            SocketAddr::from_str("127.0.0.1:6060").unwrap()
        );
        assert_eq!(cmds[0].at.as_ref().unwrap(), "n");
        assert_eq!(cmds[1].alias, "my_inlet");
        assert_eq!(
            cmds[1].from,
            SocketAddr::from_str("127.0.0.1:6061").unwrap()
        );
        assert!(cmds[1].at.is_none());

        let unnamed = r#"
            tcp_inlets:
              - from: 6060
                at: n
              - from: '6061'
        "#;
        let parsed: TcpInlets = serde_yaml::from_str(unnamed).unwrap();
        let cmds = parsed.into_commands().unwrap();
        assert_eq!(cmds.len(), 2);
        assert_eq!(
            cmds[0].from,
            SocketAddr::from_str("127.0.0.1:6060").unwrap()
        );
        assert_eq!(cmds[0].at.as_ref().unwrap(), "n");
        assert_eq!(
            cmds[1].from,
            SocketAddr::from_str("127.0.0.1:6061").unwrap()
        );
        assert!(cmds[1].at.is_none());
    }
}
