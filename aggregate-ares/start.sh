git pull
docker-compose down

cd /home/server/ares_dome/huobiaggregate/
mvn  install -Dmaven.test.skip=true
mv   /home/server/ares_dome/huobiaggregate/target/huobiaggregate-0.0.1-SNAPSHOT.jar   /home/server/ares_dome/huobiaggregate/docker/app.jar
cd  /home/server/ares_dome/huobiaggregate/docker/
docker build -t  huobiaggregate:last  .


cd  /home/server/ares_dome/okexaggregate/
mvn  install -Dmaven.test.skip=true
mv  /home/server/ares_dome/okexaggregate/target/okexaggregate-0.0.1-SNAPSHOT.jar   /home/server/ares_dome/okexaggregate/docker/app.jar
cd  /home/server/ares_dome/okexaggregate/docker/
docker build -t  okexaggregate:last  .

cd  /home/server/ares_dome/
docker-compose up -d 
