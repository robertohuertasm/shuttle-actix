# shuttle-actix

This is just a simple repo to test [Shuttle](https://www.shuttle.rs/).

This example can be found online [here](https://shuttle-actix.shuttleapp.rs).

## Api

- "/" -> will return a list of all the notes.
- "/{whatever}" -> will add a note to the database.

We're using only the `GET` verb even for inserts just for the sake of simplicity.

## Deployment

```sh
cargo shuttle deploy
```

## Run it locally

```sh
cargo shuttle run
```