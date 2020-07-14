#!/bin/sh

set -e

if [ -d election-metadata ]
then
    echo "Skipping git pull; election-metadata already exists."
else
    git clone https://github.com/ranked-vote/election-metadata
fi

mkdir -p raw-data
s3fs -o use_path_request_style raw.ranked.vote raw-data

mkdir -p reports
s3fs -o use_path_request_style reports.ranked.vote reports

