FROM debian:bookworm
RUN apt update && \
    apt install -y curl
RUN curl -fsSL https://deb.nodesource.com/setup_23.x -o /opt/nodesource_setup.sh
RUN sh /opt/nodesource_setup.sh
RUN apt install -y nodejs build-essential
RUN npm install -g nodemon
RUN curl https://sh.rustup.rs -sSf  | sh -s -- -y -q;
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install wasm-pack
COPY app /app/app
COPY components /app/components
COPY src /app/src
COPY utils /app/utils
COPY Cargo.lock /app/Cargo.lock
COPY Cargo.toml /app/Cargo.toml
COPY next.config.mjs /app/next.config.mjs
COPY nodemon.json /app/nodemon.json
COPY package-lock.json /app/package-lock.json
COPY package.json /app/package.json
COPY postcss.config.mjs /app/postcss.config.mjs
COPY tailwind.config.ts /app/tailwind.config.ts
COPY tsconfig.json /app/tsconfig.json
EXPOSE 3000
WORKDIR /app
RUN npm install
ENTRYPOINT [ "npm", "run", "dev" ]