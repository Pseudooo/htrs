use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::header_commands::HeaderCommand;
use crate::commands::service_commands::ServiceCommand;

pub enum RootCommands {
    Service(ServiceCommand),
    Call(CallServiceEndpointCommand),
    Header(HeaderCommand),
}