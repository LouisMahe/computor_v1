#!/bin/bash

cargo run -q "$1"
/usr/bin/google-chrome "poly_plot.html" >/dev/null 2>&1 &