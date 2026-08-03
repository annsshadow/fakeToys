# x_file_assemble_control

## Endpoints

- `POST /jaxrs/attachment/upload/folder/{folder}` — addAttachment
- `POST /jaxrs/attachment2/upload/folder/{folder}` — addAttachment2
- `POST /jaxrs/attachment2/upload/folder/{folder}` — addAttachmentMd5
- `GET /jaxrs/attachment2/exist/file/{fileMd5}` — checkFileExist
- `POST /jaxrs/attachment` — addAttachment_jaxrs
- `GET /jaxrs/attachment/list/folder/{id}` — listAttachment
- `GET /jaxrs/attachment/list/top` — listAttachmentTop
- `GET /jaxrs/attachment/{id}` — getAttachment
- `DELETE /jaxrs/attachment/{id}` — removeAttachment
- `DELETE /jaxrs/attachment/{id}` — deleteFile
- `PUT /jaxrs/attachment/{id}` — updateAttachment
- `GET /jaxrs/attachment/{id}/download` — getAttachmentData
- `GET /jaxrs/attachment/{id}/download/stream` — getAttachmentStream
- `PUT /jaxrs/attachment/{id}/update` — updateAttachmentData
- `GET /jaxrs/attachment2/{id}/download` — getAttachmentData2
- `GET /jaxrs/attachment2/{id}/download/stream` — getAttachmentStream2
- `POST /jaxrs/folder` — addFolder
- `GET /jaxrs/folder/list/top` — listFolderTop
- `GET /jaxrs/folder/list/top` — listTopFolder
- `GET /jaxrs/folder/{id}` — getFolder
- `DELETE /jaxrs/folder/{id}` — deleteFolder
- `PUT /jaxrs/folder/{id}` — updateFolder
- `GET /jaxrs/folder/list/{id}` — listFolder
- `GET /jaxrs/share/list` — listShare
- `GET /jaxrs/attachment/list/share/{person}` — listShareAttachment
- `GET /jaxrs/editor/list` — listEditor
- `GET /jaxrs/attachment/list/editor/{person}` — listEditorAttachment
- `GET /jaxrs/complex/folder/{id}` — listComplex
- `GET /jaxrs/attachment/{id}/image/width/{width}/height/{height}/binary/base64` — getBase64Code
- `UNKNOWN x_file_assemble_control` — clazz
