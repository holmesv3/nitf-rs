# nitf-rs

[![crates.io](https://img.shields.io/crates/v/nitf-rs)](https://crates.io/crates/nitf-rs)
[![docs](https://img.shields.io/docsrs/nitf-rs)](https://docs.rs/nitf-rs/latest/nitf_rs/)
[![Discord](https://img.shields.io/discord/1109246714721865810?label=discord&logo=discord&logoColor=white&color=blue)](https://discord.gg/Kg7NwN4XgS)

A rust NITF file interface

Current project goals are
- Finish implementing version 2.1 reading (maybe writing)
  [ ] Verify header/subheader field entries (see [`NitfField`](https://docs.rs/nitf-rs/0.1.4/nitf_rs/nitf_2_1/types/struct.NitfField.html))
    - Will likely make the `NitfField` generic for the input type, such that values could be parsed based on `enums` or a range of valid inputs.
    - Potentially add `NitfField.val` for this parsed value
  [ ] Verify all segments are parsed properly
  [ ] Implement logic for various header/subheader inputs 
      - For example, return data from an image segment as an array with the appropriate format (right now only `Complex32` supported)
- Expand for various applications of the standard (SICD as a first step)

This started as a personal project to learn the language better, and I work on 
it as I have time to. 

If you have questions, would like to contribute, or would like to request 
something be added, I made a discord server [here](https://discord.gg/Kg7NwN4XgS)
which you can join and ask/request things, or create an issue.

## Current Functionality

In my own testing, I have only been able to find files to verify parsing for:
- Header
- Image Segments
- Data Extension Segments

To the best of my knowledge, these function as expected.

There is no logic built around the inputs yet