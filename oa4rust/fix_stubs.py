import re

filepath = r'D:\WORKSPACE\fakeToys\oa4rust\crates\program_center\src\lib.rs'
with open(filepath, 'r', encoding='utf-8') as f:
    content = f.read()

# Count occurrences before
count_before = content.count('Value::Bool(true)')
print(f'Occurrences before: {count_before}')

# Category A: Delete operations - replace 'deleted: true' with 'id: id'
content = content.replace(
    '            ("deleted".to_string(), Value::Bool(true)),',
    '            ("id".to_string(), Value::String(id)),'
)

count_after_a = content.count('Value::Bool(true)')
print(f'Occurrences after Category A (delete -> id): {count_after_a}')

# Category B: Remove boolean stubs from operations that already return data
# These are cases where we have other meaningful fields alongside the boolean

# For operations that return id + boolean (insert/update patterns)
# We need to be careful to only remove the boolean line, not change other lines

# Pattern: ("id".to_string(), Value::String(id)),\n            ("X".to_string(), Value::Bool(true)),
# Replace with just the id line

# Let's do targeted replacements for each known pattern

replacements_b = [
    # captcha create
    ('            ("id".to_string(), Value::String(id)),\n            ("created".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),'),
    # code create mobile
    ('            ("id".to_string(), Value::String(id)),\n            ("sent".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),'),
    # config save
    ('            ("id".to_string(), Value::String(id)),\n            ("key".to_string(), Value::String(key)),\n            ("saved".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),\n            ("key".to_string(), Value::String(key)),'),
    # deploy web resource
    ('            ("id".to_string(), Value::String(id)),\n            ("asNew".to_string(), Value::String(as_new)),\n            ("created".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),\n            ("asNew".to_string(), Value::String(as_new)),'),
    # dict data path
    ('            ("dictFlag".to_string(), Value::String(dict_flag)),\n            ("path".to_string(), Value::String(_path)),\n            ("updated".to_string(), Value::Bool(true)),',
     '            ("dictFlag".to_string(), Value::String(dict_flag)),\n            ("path".to_string(), Value::String(_path)),'),
    # mpweixin menu update
    ('            ("id".to_string(), Value::String(id)),\n            ("saved".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),'),
    # application create
    ('            ("id".to_string(), Value::String(id)),\n            ("name".to_string(), Value::String(name)),\n            ("appId".to_string(), Value::String(app_id)),\n            ("description".to_string(), Value::String(description)),\n            ("created".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),\n            ("name".to_string(), Value::String(name)),\n            ("appId".to_string(), Value::String(app_id)),\n            ("description".to_string(), Value::String(description)),'),
    # application save
    ('            ("id".to_string(), Value::String(id)),\n            ("saved".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),'),
    # agent create
    ('            ("id".to_string(), Value::String(id)),\n            ("name".to_string(), Value::String(name)),\n            ("flag".to_string(), Value::String(flag)),\n            ("description".to_string(), Value::String(description)),\n            ("created".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),\n            ("name".to_string(), Value::String(name)),\n            ("flag".to_string(), Value::String(flag)),\n            ("description".to_string(), Value::String(description)),'),
    # agent save
    ('            ("id".to_string(), Value::String(id)),\n            ("saved".to_string(), Value::Bool(true)),',
     '            ("id".to_string(), Value::String(id)),'),
    # token threshold update
    ('            ("threshold".to_string(), Value::Number(serde_json::Number::from(threshold))),\n            ("updated".to_string(), Value::Bool(true)),',
     '            ("threshold".to_string(), Value::Number(serde_json::Number::from(threshold))),'),
]

for old, new in replacements_b:
    if old in content:
        content = content.replace(old, new)
        print(f'Replaced Category B pattern')
    else:
        print(f'WARNING: Category B pattern not found')

count_after_b = content.count('Value::Bool(true)')
print(f'Occurrences after Category B: {count_after_b}')

# Category C: Sync/event log operations - replace boolean with real data
# For these, we need to add id and other fields

# andfx_pull_sync
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'andfx', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'andfx', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("andfx".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))'''
)

count_after_c1 = content.count('Value::Bool(true)')
print(f'Occurrences after Category C1 (andfx): {count_after_c1}')

# dingding_pull_sync - has affected count, just remove synced
content = content.replace(
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("affected".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))''',
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("affected".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))'''
)

# dingding_request_pull_sync
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'dingding', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'dingding', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("dingding".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))'''
)

# dingding_sync_organization_callback
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'dingding', 'callback', $2, $3, NOW())",
            &[&uuid::Uuid::new_v4().to_string(), &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'dingding', 'callback', $2, $3, NOW())",
            &[&id, &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("dingding".to_string())),
            ("action".to_string(), Value::String("callback".to_string())),
            ("orgId".to_string(), Value::String(org_id)),
            ("eventType".to_string(), Value::String(event_type)),
        ]),
    ))))'''
)

# dingding_sync_organization_register_callback_enable - remove registered, keep enable
content = content.replace(
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("registered".to_string(), Value::Bool(true)),
            ("enable".to_string(), Value::String(enable)),
        ]),
    ))))''',
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("enable".to_string(), Value::String(enable)),
        ]),
    ))))'''
)

count_after_c2 = content.count('Value::Bool(true)')
print(f'Occurrences after Category C2 (dingding): {count_after_c2}')

# invoke operations - remove executed boolean
content = content.replace(
    '''            ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("executed".to_string(), Value::Bool(true)),''',
    '''            ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),'''
)

count_after_c3 = content.count('Value::Bool(true)')
print(f'Occurrences after Category C3 (invoke): {count_after_c3}')

# jest_clear_cache_source
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'jest', 'clear_cache', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("cleared".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'jest', 'clear_cache', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("jest".to_string())),
            ("action".to_string(), Value::String("clear_cache".to_string())),
        ]),
    ))))'''
)

# market_flag_install_or_update
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'market', 'install_or_update', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("installed".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'market', 'install_or_update', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("market".to_string())),
            ("action".to_string(), Value::String("install_or_update".to_string())),
        ]),
    ))))'''
)

# market_flag_uninstall
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'market', 'uninstall', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("uninstalled".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'market', 'uninstall', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("market".to_string())),
            ("action".to_string(), Value::String("uninstall".to_string())),
        ]),
    ))))'''
)

count_after_c4 = content.count('Value::Bool(true)')
print(f'Occurrences after Category C4 (market): {count_after_c4}')

# market_id_download - remove downloaded boolean
content = content.replace(
    '''    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("downloaded".to_string(), Value::Bool(true)),
                ]),
            ))))
        }''',
    '''    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                ]),
            ))))
        }'''
)

# qiyeweixin_pull_sync
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'qiyeweixin', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'qiyeweixin', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("qiyeweixin".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))'''
)

# qiyeweixin_send_getprivateinfo_message - remove success
content = content.replace(
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
        ]),
    ))))'''
)

# schedule_schedule_fire - remove fired
content = content.replace(
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("scheduleId".to_string(), Value::String(schedule_id)),
            ("fired".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("scheduleId".to_string(), Value::String(schedule_id)),
        ]),
    ))))'''
)

# validation_timeout_timeout - remove success
content = content.replace(
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
            ("timeout".to_string(), Value::Number(serde_json::Number::from(timeout))),
        ]),
    ))))''',
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("timeout".to_string(), Value::Number(serde_json::Number::from(timeout))),
        ]),
    ))))'''
)

# zhengwudingding_pull_sync
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'zhengwudingding', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'zhengwudingding', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("zhengwudingding".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))'''
)

# zhengwudingding_regist_callback
content = content.replace(
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("registered".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("zhengwudingding".to_string())),
        ]),
    ))))'''
)

# zhengwudingding_sync_organization_callback
content = content.replace(
    '''    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'zhengwudingding', 'callback', $2, $3, NOW())",
            &[&uuid::Uuid::new_v4().to_string(), &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))''',
    '''    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'zhengwudingding', 'callback', $2, $3, NOW())",
            &[&id, &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("zhengwudingding".to_string())),
            ("action".to_string(), Value::String("callback".to_string())),
            ("orgId".to_string(), Value::String(org_id)),
            ("eventType".to_string(), Value::String(event_type)),
        ]),
    ))))'''
)

count_after_c5 = content.count('Value::Bool(true)')
print(f'Occurrences after Category C5 (zhengwudingding): {count_after_c5}')

# Category D: Fix vip stub
content = content.replace(
    '''    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("entity".to_string(), Value::String(row.get("entity"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                    ("vip".to_string(), Value::Bool(true)),
                ]),
            ))))''',
    '''    let row = client
        .query_opt(
            "SELECT id, name, entity, vip, creator, create_time FROM x_program_module ORDER BY name LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("entity".to_string(), Value::String(row.get("entity"))),
                    ("vip".to_string(), Value::Bool(row.get("vip"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))'''
)

count_final = content.count('Value::Bool(true)')
print(f'Final occurrences: {count_final}')

with open(filepath, 'w', encoding='utf-8') as f:
    f.write(content)

print('Done!')
