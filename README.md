# Homepage
## Description
This Repository contains the files necessary to run my homepage.  It uses `rust`
as it's backend language with the web framework `rocket` and the ORM `diesel`
and for the frontend it uses `sass` for styling and the `lit` webframework for
more complicated fronted code
## Dependencies
In order to build it and deploy it you need:
- `make`
- `sassc`
- `yarn`
- A working rust installation (`stable channel`)

## Setup
The database url is read from the environment variable `DATABASE_URL` (you can
store it in `.env`). To set up the database you can use the `diesel` cli. You
can simply run `diesel migration run` to setup your database.

To build everything just run `make` (note that `make` doesn't just build the
webserver but also all the frontend code).

If you wish to run the website use `cargo run`
