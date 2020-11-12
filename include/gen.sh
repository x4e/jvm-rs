#!/bin/bash

bindgen $1.h --no-layout-tests -o $1.rs
