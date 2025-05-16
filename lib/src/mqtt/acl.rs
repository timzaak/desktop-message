use rmqtt::context::ServerContext;
use rmqtt::hook::{Handler, HookResult, Parameter, Register, ReturnType, Type};
use rmqtt::macros::Plugin;
use rmqtt::plugin::Plugin;
use rmqtt::{register, Result};
use async_trait::async_trait;
use rmqtt::types::AuthResult;

register!(AclPlugin::new);

#[derive(Plugin)]
pub struct AclPlugin {
    scx: ServerContext,
    name: String,
    register: Box<dyn Register>,
}
impl AclPlugin {
    #[inline]
    async fn new<N: Into<String>>(scx: ServerContext, name: N) -> Result<Self> {
        let name = name.into();
        let register = scx.extends.hook_mgr().register();
        Ok(Self { scx, name, register })
    }
}

#[async_trait]
impl Plugin for AclPlugin {
    #[inline]
    async fn load_config(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    async fn init(&mut self) -> Result<()> {
        self.register.add_priority(Type::ClientAuthenticate, 1, Box::new(AclHandler {})).await;
        Ok(())
    }



    #[inline]
    async fn stop(&mut self) -> Result<bool> {
        log::warn!("the default ACL plug-in, it cannot be stopped");
        //self.register.stop().await;
        Ok(false)
    }

    async fn start(&mut self) -> Result<()> {
        self.register.start().await;
        Ok(())
    }
}

struct AclHandler {
}

#[async_trait]
impl Handler for AclHandler {
    async fn hook(&self, param: &Parameter, acc: Option<HookResult>) -> ReturnType {
        match param {
            Parameter::ClientAuthenticate(connect_info) => {
                if matches!(
                    acc,
                    Some(HookResult::AuthResult(AuthResult::BadUsernameOrPassword))
                        | Some(HookResult::AuthResult(AuthResult::NotAuthorized))
                ) {
                    return (false, acc);
                }
                if connect_info.client_id().starts_with("ac_") {
                    return (false, Some(HookResult::AuthResult(AuthResult::Allow(false, None))))
                }
                return (false, Some(HookResult::AuthResult(AuthResult::NotAuthorized)))

            }
            _ => {}
        }
        (true, acc)
    }
}


