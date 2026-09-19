# x_message_assemble_communicate

## Endpoints

- `POST /api/mass` — create
- `DELETE /api/mass/{id}` — delete
- `GET /api/mass/{id}` — get
- `GET /api/mass/list/{id}/next/{count}` — list
- `GET /api/mass/enable/type` — enableType
- `GET /api/consume/list/{consume}/count/{count}` — mqList
- `PUT /api/consume/type/{type}` — mqUpdate
- `GET /api/consume/{id}/type/{type}` — mqUpdateSingle
- `GET /api/consume/list/{consume}/count/{count}` — messageListAll
- `GET /api/consume/list/{consume}/currentperson/count/{count}` — messageList
- `PUT /api/consume/type/{type}` — messageUpdate
- `GET /api/consume/{id}/type/{type}` — messageUpdateSingle
- `GET /api/im/msg/download/{id}/image/width/{width}/height/{height}` — imgFileDownloadWithWH
- `GET /api/im/msg/download/{id}` — imgFileDownload
