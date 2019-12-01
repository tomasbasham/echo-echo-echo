# echo-echo-echo [![Build Status](https://travis-ci.com/tomasbasham/echo-echo-echo.svg?branch=master)](https://travis-ci.com/tomasbasham/echo-echo-echo)

This README outlines the details of collaborating on this Rust application. A
short introduction of this app could easily go here.

## Prerequisites

You will need the following things properly installed on your computer.

* [Git](https://git-scm.com/)
* [Rust](https://www.rust-lang.org/)
* [Docker](https://www.docker.com/)

## Installation

* `git clone <repository-url>` this repository
* `cd echo-echo-echo`
* `docker build -t echo .`

## Running / Development

* `docker run --rm -it -p 3000:3000 --env-file .env echo`
* Visit your app at [http://localhost:3000](http://localhost:3000).

## Further Reading / Useful Links

* [Rust](https://www.rust-lang.org/)
