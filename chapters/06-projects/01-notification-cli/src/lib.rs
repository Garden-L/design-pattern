//! Compose the chapter factory, builder, and strategy behind one facade.
use behavioral::strategy::{price_with, Order, PricingStrategy, Regular, TenPercentOff};
use creational::{
    builder::{BuildError, ReportBuilder},
    factory_method::{EmailFactory, NotificationFactory, SmsFactory},
};
use std::{collections::HashMap, error::Error, fmt};

pub const HELP: &str = "Usage: notification-cli --channel email|sms --title TEXT --message TEXT [--pricing regular|discount]\n";

#[derive(Debug)]
pub enum CliError {
    Arguments(String),
    Build(BuildError),
}
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arguments(message) => f.write_str(message),
            Self::Build(error) => write!(f, "invalid notification: {error:?}"),
        }
    }
}
impl Error for CliError {}

pub struct NotificationFacade;
impl NotificationFacade {
    /// Validation completes before factory delivery. Prices are teaching units.
    pub fn send(
        &self,
        factory: &dyn NotificationFactory,
        strategy: &dyn PricingStrategy,
        title: &str,
        message: &str,
    ) -> Result<String, BuildError> {
        let report = ReportBuilder::new().title(title).body(message).build()?;
        let price = price_with(strategy, &Order { subtotal: 100 });
        let delivery = factory.create(&report.render()).deliver();
        Ok(format!("{delivery}\nPrice: {price}\n"))
    }
}

/// Parse flags and return output; the binary owns process I/O and exit codes.
pub fn run(args: impl IntoIterator<Item = String>) -> Result<String, CliError> {
    let args: Vec<_> = args.into_iter().collect();
    if args == ["--help"] {
        return Ok(HELP.to_owned());
    }
    let mut options = HashMap::new();
    let mut args = args.iter();
    while let Some(flag) = args.next() {
        if !matches!(
            flag.as_str(),
            "--channel" | "--title" | "--message" | "--pricing"
        ) {
            return Err(CliError::Arguments(format!("unknown option: {flag}")));
        }
        let value = args
            .next()
            .filter(|value| !value.starts_with("--"))
            .ok_or_else(|| CliError::Arguments(format!("missing value for {flag}")))?;
        if options.insert(flag.as_str(), value.as_str()).is_some() {
            return Err(CliError::Arguments(format!("duplicate option: {flag}")));
        }
    }
    let required = |flag| {
        options
            .get(flag)
            .copied()
            .ok_or_else(|| CliError::Arguments(format!("missing {flag}")))
    };
    let factory: &dyn NotificationFactory = match required("--channel")? {
        "email" => &EmailFactory,
        "sms" => &SmsFactory,
        value => return Err(CliError::Arguments(format!("unsupported channel: {value}"))),
    };
    let strategy: &dyn PricingStrategy =
        match options.get("--pricing").copied().unwrap_or("regular") {
            "regular" => &Regular,
            "discount" => &TenPercentOff,
            value => return Err(CliError::Arguments(format!("unsupported pricing: {value}"))),
        };
    NotificationFacade
        .send(
            factory,
            strategy,
            required("--title")?,
            required("--message")?,
        )
        .map_err(CliError::Build)
}
