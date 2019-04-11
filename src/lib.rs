/*
 * Copyright 2019 Joyent, Inc.
 */

//! resolver is a library for resolving DNS records for cueball.
#[allow(dead_code)] // TODO: Remove after initial dev
pub mod resolver;
extern crate chrono;
extern crate cueball;
extern crate futures;
extern crate state_machine_future;
