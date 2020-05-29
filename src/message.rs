#[allow(dead_code)]
pub enum MessageStatus {
    STARTING,
    RUNNING,
    STOPPING,
    STOPPED,
    DIED,
}

#[allow(dead_code)]
pub enum MessageOutput {
    Msg(String),
    NoMsg,
}

#[allow(dead_code)]
pub struct Message {
    pub status: MessageStatus,
    pub output: MessageOutput,
}
