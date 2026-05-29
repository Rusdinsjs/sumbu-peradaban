FROM node:22-slim

WORKDIR /app

# Copy package files first for dependency caching
COPY package*.json ./
RUN npm install

# Copy source code
COPY . .

EXPOSE 5173

CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
