use anyhow::{anyhow, Result};
{% if cookiecutter.with_subcommands %}
use clap::{Args, CommandFactory, Parser, Subcommand};
{% else %}
use clap::Parser;
{% endif %}
use clap::builder::styling::{Color, AnsiColor, Ansi256Color, Style};
{% if cookiecutter.with_tracing %}
use tracing;
use tracing_subscriber::fmt;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
{% endif %}

{% if cookiecutter.with_subcommands %}
// Create structs for each command.
#[derive(Args, Debug)]
struct ParseCmd {
    #[arg(help="Path to parse")]
    path: std::path::PathBuf,
}

// Add an execute method for it.  This does the work.
impl ParseCmd {
    {% if cookiecutter.with_tracing %}
    #[tracing::instrument]
    {% endif %}
    fn execute(&self) -> Result<()> {
        println!("parse: {:?}", self.path);

        {% if cookiecutter.with_tracing %}
        tracing::info!("message");

        {% endif  %}
        println!("do work here!");

        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a file.
    Parse(ParseCmd),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None, styles=get_styles())]
#[command(propagate_version = true)]
struct TopLevelCmd {
    {% if cookiecutter.with_tracing %}
    #[clap(long, global=true, help="Show debug logging")]
    debug: bool,

    {% endif  %}
    #[command(subcommand)]
    command: Option<Commands>,
}

impl TopLevelCmd {
    fn execute(self) -> Result<(), anyhow::Error> {
        let TopLevelCmd {
            {% if cookiecutter.with_tracing %}
            debug: _,
            {% endif %}
            command,
        } = self;

        match command {
            Some(Commands::Parse(command)) => command.execute(),
            None => {
                TopLevelCmd::command().print_long_help()?;
                // Note: clap uses an exit code of 2 when CLI parsing fails
                std::process::exit(2);
            },
        }
    }
}

{% else %}
#[derive(Parser)]
#[command(author, version, about, long_about = None, styles=get_styles())]
#[command(propagate_version = true)]
struct Cli {
    {% if cookiecutter.with_tracing %}
    #[clap(long, help="Show debug logging")]
    debug: bool,

    {% endif  %}
    #[arg(help="Path to parse")]
    path: std::path::PathBuf,
}

impl Cli {
    fn execute(&self) -> Result<()> {
        println!("path to parse: {:?}", self.path);

        {% if cookiecutter.with_tracing %}
        tracing::info!("message");

        {% endif  %}
        println!("do work here!");

        Ok(())
    }
}

{% endif %}
fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi256(Ansi256Color(180)))),
        )
        .header(
            Color::Ansi256(Ansi256Color(180)).on_default()
        )
        .literal(
            Color::Ansi256(Ansi256Color(187)).on_default()
        )
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .placeholder(
            Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::White))),
        )
}

fn main() {
    {% if cookiecutter.with_subcommands %}
    let cli = TopLevelCmd::parse();
    {% else %}
    let cli = Cli::parse();
    {% endif %}

    {% if cookiecutter.with_tracing %}
    let fmt_layer = fmt::layer()
        .with_filter(
            if cli.debug {
                LevelFilter::DEBUG
            } else {
                LevelFilter::OFF
            }
        );

    tracing_subscriber::registry()
        .with(fmt_layer)
        .init();

    {% endif %}
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let res = cli.execute();
    match res {
        Ok(()) => (),
        Err(err) => {
            println!("ERROR: {:?}", err.to_string());
            std::process::exit(2);
        }
    }
}
