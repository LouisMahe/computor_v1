#!/bin/bash

cargo run -q "$1"
open "poly_plot.html" >/dev/null 2>&1 &