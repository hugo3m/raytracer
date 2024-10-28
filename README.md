# Raytracer

Raytracer built in Rust web assembly served with NextJS, inspired from [Computer Graphics from Scratch, Gabriel Gambetta](https://gabrielgambetta.com/computer-graphics-from-scratch/index.html).

**Yes**, I am aware that the built folder should not be pushed to the repository. But it made things easier to deploy on Vercel 🙂

## Folders

* `app`: NextJS app folder
* `components`: NextJS components
* `utils`: Utils for NextJS, such as singleton engine.
* `src`: Rust files

## Running the project

```
node --version
v22.9.0
cargo --version
cargo 1.83.0-nightly (eaee77dc1 2024-09-19)
```

```
npm run dev
```

## Using docker

```
docker build --tag {tag} .
docker run -p 3000:3000
```
