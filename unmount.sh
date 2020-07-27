#!/bin/sh

set -e

umount raw-data
umount preprocessed

rmdir raw-data
rmdir preprocessed

