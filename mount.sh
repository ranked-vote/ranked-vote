#!/bin/sh

set -e

if [ -d election-metadata ]
then
    echo "Skipping election-metadata; already exists."
else
    git clone git@github.com:ranked-vote/election-metadata.git
fi

if [ -d reports ]
then
    echo "Skipping reports; already exists."
else
    git clone git@github.com:ranked-vote/reports.git
fi

mkdir -p raw-data
s3fs -o use_path_request_style raw.ranked.vote raw-data

mkdir -p preprocessed
s3fs -o use_path_request_style data.ranked.vote preprocessed

