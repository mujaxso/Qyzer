use core_types::workspace::EditorCommand;

#[allow(dead_code)]
pub struct Command {
    pub command: EditorCommand,
}

#[allow(dead_code)]
impl Command {
    pub fn new(command: EditorCommand) -> Self {
        Self { command }
    }

    pub fn execute(&self) {
        match &self.command {
            EditorCommand::OpenWorkspace { path: _ } => {
                // TODO: Actually open the workspace
            }
            EditorCommand::OpenFile { path: _ } => {
                // TODO: Actually open the file
            }
            EditorCommand::SaveActiveFile => {
                // TODO: Actually save the file
            }
        }
    }
}
