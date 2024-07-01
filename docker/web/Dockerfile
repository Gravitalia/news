FROM node:21-alpine3.18 as builder
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

USER node
WORKDIR /news

COPY ../front .

FROM builder AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

FROM builder AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
RUN pnpm run build

FROM builder as runtime
ENV NODE_ENV production

USER node

WORKDIR /

COPY --from=prod-deps /news/node_modules /news/node_modules
COPY --from=prod-deps /news/package.json package.json
COPY --from=build /news/.output .output

EXPOSE  3000
CMD     ["pnpm", "preview"]