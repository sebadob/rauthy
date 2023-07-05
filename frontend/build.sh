#!/bin/bash

rm -rf ../static/v1/*
rm -rf ../templates/html/*

mkdir ../templates/html
mkdir -p ../static/v1

npm run build

pages=(
"../templates/html/*.html"
"../templates/html/oidc/*.html"
"../templates/html/users/*.html"
"../templates/html/users/{id}/reset/*.html"
)

for folder in "${pages[@]}"; do
    for html in $folder; do
      # for pre-rendering colors
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
      # for the nonce in the CSP for script files
      sed -i 's/<link /<link nonce="{{ nonce }}" /g' "$html"
      sed -i 's/<script>/<script nonce="{{ nonce }}">/g' "$html"
    done;
done

# Since we cleanup the whole static folder, copy over the book again at the end
#mkdir ../static/v1/book
#cp -r ../docs/* ../static/v1/book
