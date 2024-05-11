use tracing_subscriber::util::SubscriberInitExt;

#[cfg(feature = "log")]
use crate::log;
use crate::Context;

#[derive(Debug, Default)]
pub struct Builder<L>
where
    L: SubscriberInitExt + Default,
{
    pub context: Option<Context>,
    pub log: Option<L>,
}

impl<L> Builder<L>
where
    L: SubscriberInitExt + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }

    #[cfg(feature = "log")]
    pub fn with_log(mut self, log: L) -> Self {
        self.log = Some(log);
        self
    }

    pub fn install(self) {
        #[cfg(feature = "log")]
        {
            let context = self.context.unwrap_or_else(Context::load);
            log::install(&context, self.log);
        }
    }
}
