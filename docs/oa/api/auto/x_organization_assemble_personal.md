# x_organization_assemble_personal

## Endpoints

- `PUT /jaxrs/person/password` — changePassword
- `GET /jaxrs/person` — getPerson
- `PUT /jaxrs/person` — updatePerson
- `PUT /jaxrs/person/icon` — changeIcon
- `GET /jaxrs/reset/check/password/{password}` — checkPassword
- `GET /jaxrs/person/icon` — getPersonIcon
- `GET /jaxrs/icon/{person}` — getIcon
- `GET /jaxrs/regist/mode` — getRegisterMode
- `GET /jaxrs/regist/captcha/width/{width}/height/{height}` — getRegisterCaptcha
- `GET /jaxrs/regist/code/mobile/{mobile}` — createRegisterCode
- `GET /jaxrs/regist/check/name/{name}` — checkRegisterName
- `GET /jaxrs/regist/check/password/{password}` — checkRegisterPassword
- `GET /jaxrs/regist/check/mobile/{mobile}` — checkRegisterMobile
- `POST /jaxrs/regist` — register
- `PUT /jaxrs/reset` — resetPassword
- `GET /jaxrs/reset/check/credential/{credential}` — checkCredentialOnResetPassword
- `GET /jaxrs/reset/check/password/{password}` — checkPasswordOnResetPassword
- `GET /jaxrs/reset/code/credential/{credential}` — createCodeOnResetPassword
- `POST /jaxrs/reset/password/anonymous` — setPasswordAnonymous
- `GET /jaxrs/custom/{name}` — getUserData
- `PUT /jaxrs/custom/{name}` — putUserData
- `DELETE /jaxrs/custom/{name}` — deleteUserData
- `GET /jaxrs/definition/{name}` — getPublicUserData
- `PUT /jaxrs/definition/{name}` — putPublicUserData
- `DELETE /jaxrs/definition/{name}` — deletePublicUserData
- `GET /jaxrs/empower/list/currentperson` — getMyEmPower
- `GET /jaxrs/empower/list/to` — getReceiveEmPower
- `POST /jaxrs/empower` — createEmPower
- `PUT /jaxrs/empower/{id}` — editEmPower
- `DELETE /jaxrs/empower/{id}` — deleteEmPower
- `GET /jaxrs/empowerlog/list/currentperson` — getMyEmPowerLog
- `GET /jaxrs/empowerlog/list/to` — getReceiveEmPowerLog
- `POST /jaxrs/empowerlog/list/to/currentperson/paging/{page}/size/{size}` — listToCurrentPersonPaging
- `POST /jaxrs/empowerlog/list/currentperson/paging/{page}/size/{size}` — listWithCurrentPersonPaging
- `UNKNOWN x_organization_assemble_personal` — clazz
