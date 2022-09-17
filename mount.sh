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

aws s3 cp s3://raw.ranked.vote raw-data --recursive
aws s3 cp s3://data.ranked.vote preprocessed --recursive

