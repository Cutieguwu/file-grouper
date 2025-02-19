#!~/.pyenv/versions/3.11.6/bin/python
#
# Copyright (c) 2024 Cutieguwu | Olivia Brooks
#
# -*- coding: utf-8 -*-
# @Title: file_grouper
# @Author: Cutieguwu | Olivia Brooks
# @Description: Simple Math practise helper.
#
# @Script: file_grouper.py
# @Date Created: 22 Jan, 2025
# @Last Modified: 22 Jan, 2025
# @Last Modified by: Cutieguwu | Olivia Brooks
# --------------------------------------------

from os import path, makedirs
from pathlib import Path


BASE_PATH = Path(f'{path.dirname(__file__)}/../dummy')

title_groups: list[str] = []

# Use *.mkv to determine groups.
for file in BASE_PATH.glob('*.mkv'):
    title_groups.append(path.basename(file.with_suffix('')))

title_groups = sorted(title_groups, key=lambda x: (len(x), x))
title_groups.reverse()

for title in title_groups:
    group_path = f'{BASE_PATH.name}/{title}'

    # Create a dir if dir doesn't exist.
    # Use if statement instead of handling OSError.
    if not path.exists(group_path):
        print(f'$BASE_PATH/{title}/ does not exist. Creating path...')
        makedirs(group_path)
    
    for file in BASE_PATH.glob(f'{title}*'):
        # Find file entries, ignore dir.
        if path.isfile(file):
            # Move file to grouping dir.
            print(f'Moving "{path.basename(file)}" -> $BASE_PATH/{title}/')
            Path(file).rename(f'{group_path}/{path.basename(file)}')
