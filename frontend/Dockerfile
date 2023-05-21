FROM node:18-slim
RUN npm install -g pnpm

WORKDIR /frontend
COPY pnpm-lock.yaml ./
RUN pnpm fetch

COPY . .
RUN pnpm install --offline

EXPOSE 5173