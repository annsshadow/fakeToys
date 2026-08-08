$file = "D:\WORKSPACE\fakeToys\oa4rust\crates\processplatform_assemble_bam\src\lib.rs"
$content = Get-Content $file -Raw
$old = 'Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))'
$new = 'Err(AppError::NotImplemented)'
$content = $content.Replace($old, $new)
Set-Content $file -Value $content -NoNewline
Write-Host "Done"
