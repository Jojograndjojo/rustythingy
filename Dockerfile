FROM balenalib/raspberry-pi-alpine

RUN apk add --no-cache python3 py3-pip build-base python3-dev
RUN pip3 install RPI.GPIO flask

WORKDIR /usr/src/app

COPY python_libs ./python_libs/
COPY server.py ./
RUN chmod +x ./server.py

ENV PORT 5000
EXPOSE 5000

CMD [ "python3", "server.py", "runserver", "-h", "0.0.0.0"]