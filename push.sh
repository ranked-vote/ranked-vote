#!/bin/sh

set -e

aws s3 sync raw-data s3://raw.ranked.vote
aws s3 sync preprocessed s3://data.ranked.vote
