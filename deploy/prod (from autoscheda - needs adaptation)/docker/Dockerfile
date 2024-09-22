FROM eclipse-temurin:17-jre-alpine

RUN addgroup --system autoscheda 
RUN adduser --system --ingroup autoscheda autoscheda

COPY src/main/resources/static static

COPY target/autoscheda*.jar app.jar

USER autoscheda:autoscheda
ENTRYPOINT ["java","-jar","/app.jar","init","autoscheda_postgres","src"]