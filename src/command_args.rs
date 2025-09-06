use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::global_header_commands::GlobalHeaderCommand;
use crate::commands::service_commands::ServiceCommand;

pub enum RootCommands {
    Service(ServiceCommand),
    Call(CallServiceEndpointCommand),
    Header(GlobalHeaderCommand),
}