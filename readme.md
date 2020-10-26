# Description

Simple web request profiler written in rust

## Usage
Jay Madden Cox <jaymaddencox@gmail.com>
Cloudflare systems assignment

USAGE:
    hello_cargo [OPTIONS] --url <url>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --profile <profile>    Amount of times to profile a given URL
    -u, --url <url>            Url to profile

### Linktree api

50 Requests made to: https://linktree.jaymaddencox.workers.dev/links
The Average request time was: 90.51063829787235ms
The lowest request time: 61ms
The highest request time: 150ms
The median request times is 88ms
The lowest request size: 904bytes
The highest request size: 1029bytes
Non 200 error codes: []
Number of failed requests: 3

### Github 
50 Requests made to: https://github.com
The Average request time was: 75.9ms
The lowest request time: 60ms
The highest request time: 210ms
The median request times is 71ms
The lowest request size: 103bytes
The highest request size: 103bytes
Non 200 error codes: [301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301]
Number of failed requests: 0


### ptsv2
50 Requests made to: http://ptsv2.com/
The Average request time was: 89.12244897959184ms
The lowest request time: 70ms
The highest request time: 193ms
The median request times is 87ms
The lowest request size: 4825bytes
The highest request size: 4829bytes
Non 200 error codes: []
Number of failed requests: 1