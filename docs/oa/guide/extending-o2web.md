# Extending o2web

This section explains how to add a new frontend component to `o2web`.

## Directory Layout

Each component lives under `oa/o2web/source/x_component_{Name}/`:

```
source/x_component_{Name}/
  Main.js              # MWF application entry point
  lp/                   # Language packs
    zh-cn/              # Simplified Chinese
    en/                 # English (optional)
  $Main/                # Compiled assets and styles
  applications.json     # Component registry (optional)
```

## Main.js

`Main.js` defines an MWF class that extends `MWF.xApplication.Common.Main`:

```javascript
MWF.xApplication.{Name}.Main = new Class({
    Extends: MWF.xApplication.Common.Main,
    Implements: [Options, Events],

    options: {
        "style": "default",
        "name": "{Name}",
        "icon": "icon.png",
        "title": MWF.xApplication.{Name}.LP.title
    },

    onQueryLoad: function(){
        this.lp = MWF.xApplication.{Name}.LP;
        this.restActions = MWF.Actions.get("{backend_module_name}");
    },

    loadApplication: function(callback){
        this.content.loadCss("../x_component_{Name}/$Main/"+this.options.style+"/style.css");
        this.createNode();
        this.loadApplicationContent();
        if (callback) callback();
    }
});
```

## Language Packs

Place translation files under `lp/{locale}/`. Language-pack keys are loaded at runtime via `MWF.xApplication.{Name}.LP`.

## Build Integration

The gulp build reads `gulpapps.js` to discover components. To register a new component:

1. Add an entry in `gulpapps.js` pointing to the component's source directory.
2. Ensure the component's `Main.js` follows the MWF class convention.
3. Run `npm run build` to compile.

## Step-by-Step: Adding a New Component

1. **Create the component directory**:
   ```
   mkdir source/x_component_{Name}
   ```
2. **Create `Main.js`** with the MWF class skeleton.
3. **Create language packs** under `lp/zh-cn/` and optionally `lp/en/`.
4. **Register in `gulpapps.js`** so the gulp build includes it.
5. **Build**:
   ```bash
   cd oa/o2web
   npm install
   npm run build
   ```
6. **Deploy** the `dest/` output to `o2server/servers/webServer/`.

## Reference Example

`x_component_Org` demonstrates:
- MWF class inheritance from `Common.Main`
- REST action binding via `MWF.Actions.get("x_organization_assemble_control")`
- Language-pack structure under `lp/`
- Style loading from `$Main/`
