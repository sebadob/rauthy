// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use crate::from_pg_row::impl_from_pg_row;
use syn::{DeriveInput, parse_macro_input};

mod from_pg_row;

// The idea with this macro impl is that it behaves exactly the same as `hiqlite::macros::FromRow`.
// The important part about it is the `column` attribute.
#[proc_macro_derive(FromPgRow, attributes(column))]
pub fn from_pg_row(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // TODO could be reworked into returning a result, which would make the impl a bit cleaner
    impl_from_pg_row(parse_macro_input!(input as DeriveInput))
}
