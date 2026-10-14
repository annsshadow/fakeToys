# Extending o2server

This section explains how to add a new backend module to `o2server`.

## Naming Convention

Modules follow the pattern `x_{domain}_{layer}`:

| Layer | Suffix | Example |
|-------|--------|---------|
| Entity | `_core_entity` | `x_organization_core_entity` |
| Express | `_core_express` | `x_organization_core_express` |
| Assemble Control | `_assemble_control` | `x_organization_assemble_control` |
| Assemble Surface | `_assemble_surface` | `x_organization_assemble_surface` |
| Service Processing | `_service_processing` | `x_processplatform_service_processing` |
| Designer | `_assemble_designer` | `x_processplatform_assemble_designer` |

## Directory Layout

```
x_{domain}_core_entity/
  pom.xml
  src/main/java/com/x/{domain}/entity/
    {Entity}.java

x_{domain}_core_express/
  pom.xml
  src/main/java/com/x/{domain}/express/
    {Entity}Express.java

x_{domain}_assemble_control/
  pom.xml
  src/main/java/com/x/{domain}/assemble/control/
    Business.java
    {Resource}JaxrsFilter.java
    jaxrs/
      action/
        {Resource}/
          Action{Operation}.java
```

## Package Structure

Java packages mirror the module name:

```
com.x.{domain}.entity      — JPA entities
com.x.{domain}.express     — Express script wrappers
com.x.{domain}.assemble.control — Business logic and REST filters
com.x.{domain}.assemble.surface  — Presentation rendering
```

## Step-by-Step: Adding a New Module

1. **Create the module directory** under `oa/o2server/` using the naming convention.
2. **Create `pom.xml`**:
   - Inherit from the parent POM (`oa/o2server/pom.xml`).
   - Set `artifactId` to the module directory name.
   - Declare dependencies on `x_base_core_project` and any entity modules you consume.
3. **Add entity classes** in `src/main/java/com/x/{domain}/entity/`:
   - Annotate with `@Entity` and `@Table`.
   - Extend `AbstractPersistence` or implement `Persistable`.
4. **Add Express wrappers** (optional) in `x_{domain}_core_express`:
   - One `*Express.java` per entity, exposing script-accessible methods.
5. **Add assemble control** in `x_{domain}_assemble_control`:
   - `Business.java` — orchestration logic.
   - `*JaxrsFilter.java` — REST endpoint entry points.
   - `jaxrs/action/{resource}/Action*.java` — individual action classes.
6. **Register in parent POM**:
   - Add the new module directory to `<modules>` in `oa/o2server/pom.xml`.
7. **Build and verify**:
   ```bash
   cd oa/o2server
   mvn clean package -DskipTests
   ```

## Reference Example

`x_organization_assemble_control` is a mature module with:
- Entity layer: `x_organization_core_entity`
- Express layer: `x_organization_core_express`
- Assemble control: REST filters under `jaxrs/` with action classes per resource
