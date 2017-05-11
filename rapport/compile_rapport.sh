#!/bin/bash
pandoc --template rapport.latex -s $1 -o rapport.pdf --toc
