// Rauthy - OpenID Connect and Single Sign-On IdP
// Copyright (C) 2023 Sebastian Dobe <sebastiandobe@mailbox.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::str::FromStr;

pub mod constants;
pub mod error_response;
pub mod password_hasher;
pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbType {
    Sqlite,
    Postgres,
    // Mysql,
}

impl FromStr for DbType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = if s.starts_with("sqlite:") {
            Self::Sqlite
        } else if s.starts_with("postgresql://") {
            Self::Postgres
        } else {
            panic!("You provided an unknown database type, please check the DATABASE_URL");
        };

        Ok(res)
    }
}
