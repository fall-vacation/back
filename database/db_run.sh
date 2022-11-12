docker run -d \
  --name fall_db \
  --pull always \
  -p 5432:5432 \
  redheadset/fall-vacation-db:latest