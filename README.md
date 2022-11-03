# Spawner
This is a web crawler made entirely in Rust. The main objective is to rewrite
the shitty code that I made as `dist-crawler`, of which is a distributed crawler
communicating through RPC protocol. The latter was made in Go, so I first thought
that the new version could've been made in this language as well. However, I do
work full time with Go, and I do want to exercise my skills in other tools that I
like, so Rust is the way to go.

Also, we'll be using gRPC for intra worker communication, since it's more like a
machine based communication protocol than human readable like HTTP REST. For storing
the data from the webpages response, we'll be storing it in a binary file instead of
using a database at first. We'll be storing the webpages data using Flatbuffers
as a binary serializer.

## High-level
![](assets/01-spawner-highlevel.png)

## Requirements
Just `cargo`, and `protobuf`

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ brew install protobuf
```

## Usage

```
usage: spawner [OPTIONS] --mode MODE COMMAND

OPTIONS:

--start-url\tURL where spawner will crawl first
--worker\tAddress to worker with port, if using distributed mode

MODE:

standalone\tSingle node executor
distributed\tMultiple node executor, must have --worker OPT set

COMMAND:

start\tWell, start crawling, right?
```
