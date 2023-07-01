#!/bin/bash

npm run build

mkdir ../templates/html
mkdir -p ../static/v1
rm -rf ../static/v1/*
rm -rf ../templates/html/*

cp -r build/* ../static/v1/

cp build/index.html ../templates/html/index.html

for file in $(find build -name *.html); do
  cp "$file" ../templates/html/
done

for html in ../templates/html/*; do
  sed -i 's/#6b3d99;/{{ col_act1 }};/g' "$html"
  sed -i 's/#714d99;/{{ col_act1a }};/g' "$html"
  sed -i 's/#388c51;/{{ col_act2 }};/g' "$html"
  sed -i 's/#4d8c62;/{{ col_act2a }};/g' "$html"
  sed -i 's/#3d5d99;/{{ col_acnt }};/g' "$html"
  sed -i 's/#36486b;/{{ col_acnta }};/g' "$html"
  sed -i 's/#43993d;/{{ col_ok }};/g' "$html"
  sed -i 's/#993d49;/{{ col_err }};/g' "$html"
  sed -i 's/#545454;/{{ col_glow }};/g' "$html"
  sed -i 's/#b2b2b2;/{{ col_gmid }};/g' "$html"
  sed -i 's/#f2f2f2;/{{ col_ghigh }};/g' "$html"
  sed -i 's/#383838;/{{ col_text }};/g' "$html"
  sed -i 's/#f7f7f7;/{{ col_bg }};/g' "$html"
done;
