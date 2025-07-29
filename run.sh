#!/bin/bash

./computor_v1 "$1"
/usr/bin/google-chrome "poly_plot.html" >/dev/null 2>&1 &