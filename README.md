# bevy_channel_trigger

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_channel_trigger)](https://crates.io/crates/bevy_channel_trigger)
[![docs.rs](https://docs.rs/bevy_channel_trigger/badge.svg)](https://docs.rs/bevy_channel_trigger)

Send events via a channel from anywhere (eg. web-dom, c-ffi) to Bevy Observers. 
Inspired by [bevy_crossbeam_event](https://github.com/johanhelsing/bevy_crossbeam_event) but using [flume](https://github.com/zesterer/flume) instead of `crossbeam` as the underlying efficient unbounded channel and delivering the events via Bevy Observers instead of `EventReader`. Furthermore we lint to be guaranteed panic free.

Example uses: 

* [bevy_web_drop_image_as_sprite](https://github.com/rustunit/bevy_web_drop_image_as_sprite)