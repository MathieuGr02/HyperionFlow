use thiserror::Error;

pub enum DML {
    Insert,
}

pub enum Expr {
    Col(String),
}

#[derive(Debug)]
pub enum Query {
    Insert,
    Raw(String),
}

#[derive(Debug, Error)]
pub enum ParseError {}

impl Query {}
