#[macro_export]
macro_rules! lazy_error {
    ($($input:tt)*) => {
        Err($crate::errors::OsoError::Custom {
            message: format!($($input)*),
        })
    };
}

macro_rules! check_messages {
    ($core_obj:expr) => {
        while let Some(message) = $core_obj.next_message() {
            //: polar_core::messages::Message
            match message.kind {
                ::polar_core::messages::MessageKind::Print => ::tracing::debug!("{}", &message.msg),
                ::polar_core::messages::MessageKind::Warning => {
                    ::tracing::info!("{}", &message.msg)
                }
            }
        }
        true
    };
}
