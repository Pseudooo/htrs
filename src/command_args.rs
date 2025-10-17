use crate::commands::call_command::CallServiceEndpointCommand;
use crate::commands::delete_command::DeleteCommand;
use crate::commands::edit_command::EditCommand;
use crate::commands::global_header_commands::GlobalHeaderCommand;
use crate::commands::new_command::NewCommand;
use crate::commands::service_commands::ServiceCommand;

pub enum RootCommands {
    Service(ServiceCommand),
    Call(CallServiceEndpointCommand),
    Header(GlobalHeaderCommand),

    New(NewCommand),
    Edit(EditCommand),
    Delete(DeleteCommand),
}