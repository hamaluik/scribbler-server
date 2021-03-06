# Scribbler

[![GitHub license](https://img.shields.io/badge/license-Apache%202-blue.svg?style=flat-square)](https://raw.githubusercontent.com/hamaluik/scribbler-server/master/LICENSE) [![Travis](https://img.shields.io/travis/rust-lang/rust/master.svg?style=flat-square)](https://travis-ci.org/hamaluik/scribbler-server.svg?branch=master)

_Scribbler_ is a self-hosted, end-to-end encrypted note taking app which aims to be in a similar realm to [Standard Notes](https://standardnotes.org/) but with default [Markdown](https://standardnotes.org/) and entirely self-hosted.

## Motivation

I prefer to self-host as many things as I can and I can't find a suitable simple note-taking application which uses Markdown for formatting, is always in sync, and does end-to-end encryption:

* [Laverna](https://laverna.cc/) is [dead](https://github.com/Laverna/laverna/issues/971#issuecomment-411423965)
* [Turtl](https://turtlapp.com/) doesn't have a web interface and lacks an iOS app (and uses a tech stack I'm completely unfamiliar with)
* [Paperwork](http://paperwork.rocks/) is going through major changes and doesn't seem to support Markdown (and I find the editor somewhat slow and heavy)
* I find [OpenNote](https://github.com/FoxUSA/OpenNote)'s interface awkward and it doesn't include encryption
* [Leanote](https://leanote.com/) is a bit heavy, installation instructions are vague, and it doesn't seem to support syncing between things?

Ultimately this project is my Goldilocks solution for my own note-taking needs.

# Scribbler Server

This repository holds the server, which is built with [Rust](https://www.rust-lang.org/en-US/), and [Rocket](https://rocket.rs/).
