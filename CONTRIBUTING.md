# Contributing

Thanks for your interest in contributing to fs0x!

This document explains the process to contribute to the project.

## Table of Contents

- [GitHub Workflow](#github-workflow)
  - [Fork the repository](#fork-the-repository)
  - [Create a pull request](#create-a-pull-request)
  - [Update your pull request](#update-your-pull-request)


## GitHub Workflow

The recommended workflow is to fork the repository and open pull requests from your fork.

### Fork the repository

- Click the Fork button on GitHub
- Clone your fork
- Add the upstream repository as a new remote

```shell
# Clone repository
git clone git@github.com:$YOUR_GITHUB_USER/fs0x.git

# Add upstream origin
git remote add upstream git@github.com:quartz-technology/fx0x.git
```

### Create a pull request

- Create a new branch
- Make your changes
- Commit your changes
- Push to a branch in your fork
- Create a pull request to merge upstream

```shell
# Create a new feature branch
git checkout -b my_feature_branch

# Make changes to your branch
# ...

# Commit changes - remember to sign!
git commit -s

# Push your new feature branch
git push my_feature_branch

# Create a new pull request from https://github.com/quartz-technology/fx0x
```

### Update your pull request

- Go to the main branch
- Pull changes from the upstream repo
- Go back to your branch
- Rebase
- Push to update your PR

```shell
# Checkout main branch
git checkout main

# Update your fork's main branch from upstream
git pull upstream main

# Checkout your feature branch
git checkout my_feature_branch

# Rebase your feature branch changes on top of the updated main branch
git rebase main

# Update your pull request with latest changes
git push -f my_feature_branch
```