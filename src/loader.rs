/*
    This file is part of aaa.

    aaa is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    aaa is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with aaa.  If not, see <https://www.gnu.org/licenses/>.
*/
use anyhow::Result;
use atty::Stream;
use rs3a;
use rust_embed::Embed;
use std::{io, str::FromStr};

#[derive(Embed)]
#[folder = "art"]
pub struct BuiltIn;

fn get_embed_literal(name: &str) -> Option<String> {
    if let Some(file) = BuiltIn::get(name) {
        String::from_utf8(file.data.to_vec()).ok()
    } else {
        None
    }
}

pub fn get_embed(name: &str) -> Option<String> {
    if let Some(file) = get_embed_literal(name) {
        return Some(file);
    }
    match name.strip_prefix(".3a") {
        Some(name) => get_embed_literal(name),
        None => get_embed_literal(&(String::from(name) + ".3a")),
    }
}

fn looks_like_path(name: &str) -> bool {
    name.contains("\\") || name.contains("/")
}

pub fn load(file: &Option<String>) -> Result<rs3a::Art> {
    if let Some(file) = file {
        if !looks_like_path(&file) {
            if let Some(text) = get_embed(&file) {
                return Ok(rs3a::Art::from_str(&text)?);
            }
        }
        return Ok(rs3a::Art::from_file(file)?);
    }
    if atty::isnt(Stream::Stdin) {
        Ok(rs3a::Art::from_reader(io::stdin())?)
    } else {
        Err(anyhow::Error::msg(
            "ether <file> argument must be provided or data must be piped to stdin",
        ))
    }
}
