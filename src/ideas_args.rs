use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct IdeaArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// List all ideas
    List,
    
    /// Add an idea
    Add(AddCommand),

    /// Clear all ideas
    Clear,
}

#[derive(Debug, Args)]
pub struct AddCommand {
    pub idea: String,
}
