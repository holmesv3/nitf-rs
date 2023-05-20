# nitf-rs

[![crates.io](https://img.shields.io/crates/v/nitf-rs)](https://crates.io/crates/nitf-rs)
[![docs](https://img.shields.io/docsrs/nitf-rs)](https://docs.rs/nitf-rs/latest/nitf_rs/)
[![Discord](https://img.shields.io/discord/1109246714721865810?label=discord&logo=discord&logoColor=white&color=blue)](https://discord.gg/Kg7NwN4XgS)

A rust NITF file interface

Current project goals are
- Finish implementing version 2.1 reading, maybe writing
- Implement some form of 'cloud reading' to fetch files from something like s3 bucket
  - Could provide more exhaustive testing, as an actual file could be parsed without needing to store it locally
- Expand into a SICD file reader 
- Provide `Python` bindings 

This started as a personal project to learn the language better, and I work on 
it as I have time to. 

If you have questions, would like to contribute, or would like to request 
something be added, I made a discord server [here](https://discord.gg/Kg7NwN4XgS)
which you can join and ask/request things, or create an issue.