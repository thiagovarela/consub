FROM node:18 AS build-env

RUN curl -f https://get.pnpm.io/v6.16.js | node - add --global pnpm

WORKDIR /app

COPY package.json pnpm-lock.yaml postcss.config.cjs svelte.config.js tailwind.config.cjs tsconfig.json vite.config.ts ./
COPY .svelte-kit/tsconfig.json .svelte-kit/ambient.d.ts .sveltekit/

RUN pnpm install

COPY ./src ./src

RUN pnpm run build

RUN pnpm prune --prod

FROM gcr.io/distroless/nodejs18-debian11:debug
WORKDIR /app
COPY package.json ./
COPY --from=build-env /app/node_modules /app/node_modules

COPY --from=build-env /app/build /app/build
COPY ./static/ /app/build/static

EXPOSE 3000
CMD ["/app/build"]