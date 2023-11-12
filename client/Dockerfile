FROM oven/bun:latest

WORKDIR /app

COPY . .
RUN bun install
RUN bun run build

EXPOSE 4173

CMD ["bun", "run", "preview", "--host"]
