FROM node:20.10.0 as build

WORKDIR /app

COPY package*.json ./

RUN npm ci

COPY . .

RUN npm run build

FROM nginx:alpine

RUN sed -i 's/80/8080/g' /etc/nginx/conf.d/default.conf
RUN sed -i 's/index  index.html index.htm;/try_files $uri \/index.html;/g' /etc/nginx/conf.d/default.conf
RUN chmod 777 /var/cache/nginx

COPY --from=build /app/build /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]
