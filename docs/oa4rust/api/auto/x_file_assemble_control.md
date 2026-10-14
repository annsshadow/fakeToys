# x_file_assemble_control

## Endpoints

- `POST /api/attachment/upload/folder/{folder}` — addAttachment
- `POST /api/attachment2/upload/folder/{folder}` — addAttachment2
- `POST /api/attachment2/upload/folder/{folder}` — addAttachmentMd5
- `GET /api/attachment2/exist/file/{fileMd5}` — checkFileExist
- `POST /api/attachment` — addAttachment_jaxrs
- `GET /api/attachment/list/folder/{id}` — listAttachment
- `GET /api/attachment/list/top` — listAttachmentTop
- `GET /api/attachment/{id}` — getAttachment
- `DELETE /api/attachment/{id}` — removeAttachment
- `DELETE /api/attachment/{id}` — deleteFile
- `PUT /api/attachment/{id}` — updateAttachment
- `GET /api/attachment/{id}/download` — getAttachmentData
- `GET /api/attachment/{id}/download/stream` — getAttachmentStream
- `PUT /api/attachment/{id}/update` — updateAttachmentData
- `GET /api/attachment2/{id}/download` — getAttachmentData2
- `GET /api/attachment2/{id}/download/stream` — getAttachmentStream2
- `POST /api/folder` — addFolder
- `GET /api/folder/list/top` — listFolderTop
- `GET /api/folder/list/top` — listTopFolder
- `GET /api/folder/{id}` — getFolder
- `DELETE /api/folder/{id}` — deleteFolder
- `PUT /api/folder/{id}` — updateFolder
- `GET /api/folder/list/{id}` — listFolder
- `GET /api/share/list` — listShare
- `GET /api/attachment/list/share/{person}` — listShareAttachment
- `GET /api/editor/list` — listEditor
- `GET /api/attachment/list/editor/{person}` — listEditorAttachment
- `GET /api/complex/folder/{id}` — listComplex
- `GET /api/attachment/{id}/image/width/{width}/height/{height}/binary/base64` — getBase64Code
- `UNKNOWN x_file_assemble_control` — clazz
