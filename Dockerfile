FROM armhf/ubuntu

WORKDIR /usr/src/app

COPY build ./rustythingy
RUN chmod +x ./rustythingy

ENV PORT 8082
EXPOSE 8082

CMD ["./rustythingy"]