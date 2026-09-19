# x_organization_assemble_personal

## Endpoints

- `PUT /api/person/password` — changePassword
- `GET /api/person` — getPerson
- `PUT /api/person` — updatePerson
- `PUT /api/person/icon` — changeIcon
- `GET /api/reset/check/password/{password}` — checkPassword
- `GET /api/person/icon` — getPersonIcon
- `GET /api/icon/{person}` — getIcon
- `GET /api/regist/mode` — getRegisterMode
- `GET /api/regist/captcha/width/{width}/height/{height}` — getRegisterCaptcha
- `GET /api/regist/code/mobile/{mobile}` — createRegisterCode
- `GET /api/regist/check/name/{name}` — checkRegisterName
- `GET /api/regist/check/password/{password}` — checkRegisterPassword
- `GET /api/regist/check/mobile/{mobile}` — checkRegisterMobile
- `POST /api/regist` — register
- `PUT /api/reset` — resetPassword
- `GET /api/reset/check/credential/{credential}` — checkCredentialOnResetPassword
- `GET /api/reset/check/password/{password}` — checkPasswordOnResetPassword
- `GET /api/reset/code/credential/{credential}` — createCodeOnResetPassword
- `POST /api/reset/password/anonymous` — setPasswordAnonymous
- `GET /api/custom/{name}` — getUserData
- `PUT /api/custom/{name}` — putUserData
- `DELETE /api/custom/{name}` — deleteUserData
- `GET /api/definition/{name}` — getPublicUserData
- `PUT /api/definition/{name}` — putPublicUserData
- `DELETE /api/definition/{name}` — deletePublicUserData
- `GET /api/empower/list/currentperson` — getMyEmPower
- `GET /api/empower/list/to` — getReceiveEmPower
- `POST /api/empower` — createEmPower
- `PUT /api/empower/{id}` — editEmPower
- `DELETE /api/empower/{id}` — deleteEmPower
- `GET /api/empowerlog/list/currentperson` — getMyEmPowerLog
- `GET /api/empowerlog/list/to` — getReceiveEmPowerLog
- `POST /api/empowerlog/list/to/currentperson/paging/{page}/size/{size}` — listToCurrentPersonPaging
- `POST /api/empowerlog/list/currentperson/paging/{page}/size/{size}` — listWithCurrentPersonPaging
- `UNKNOWN x_organization_assemble_personal` — clazz
