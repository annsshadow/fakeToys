# x_message_assemble_communicate

## Endpoints

- `POST /jaxrs/mass` — create
- `DELETE /jaxrs/mass/{id}` — delete
- `GET /jaxrs/mass/{id}` — get
- `GET /jaxrs/mass/list/{id}/next/{count}` — list
- `GET /jaxrs/mass/enable/type` — enableType
- `GET /jaxrs/consume/list/{consume}/count/{count}` — mqList
- `PUT /jaxrs/consume/type/{type}` — mqUpdate
- `GET /jaxrs/consume/{id}/type/{type}` — mqUpdateSingle
- `GET /jaxrs/consume/list/{consume}/count/{count}` — messageListAll
- `GET /jaxrs/consume/list/{consume}/currentperson/count/{count}` — messageList
- `PUT /jaxrs/consume/type/{type}` — messageUpdate
- `GET /jaxrs/consume/{id}/type/{type}` — messageUpdateSingle
- `GET /jaxrs/im/msg/download/{id}/image/width/{width}/height/{height}` — imgFileDownloadWithWH
- `GET /jaxrs/im/msg/download/{id}` — imgFileDownload
