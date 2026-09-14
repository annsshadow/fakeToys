use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessDefinition {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub edition: Edition,
    pub activities: Vec<Activity>,
    #[serde(default, alias = "routeList")]
    pub routes: Vec<ProcessRoute>,
    #[serde(default, alias = "field_permissions")]
    pub field_permissions: Vec<FieldPermission>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edition {
    #[serde(alias = "id")]
    pub code: String,
    pub name: String,
    pub number: f64,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub id: String,
    #[serde(alias = "type")]
    pub activity_type: ActivityType,
    #[serde(default)]
    pub name: String,
    #[serde(default, alias = "routes")]
    pub route_list: Vec<String>,
    #[serde(default, alias = "assignees")]
    pub task_identity_list: Vec<String>,
    #[serde(default)]
    pub edition: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    Agent,
    Begin,
    Cancel,
    Choice,
    Condition,
    Delay,
    Embed,
    End,
    Invoke,
    Manual,
    Merge,
    Parallel,
    Publish,
    Service,
    Split,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessRoute {
    pub id: String,
    pub from: String,
    #[serde(alias = "activity")]
    pub to: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub waypoints: Vec<Waypoint>,
    #[serde(default)]
    pub edition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldPermission {
    pub field: String,
    pub activity: String,
    #[serde(default)]
    pub readable: bool,
    #[serde(default)]
    pub writable: bool,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstManualTask {
    pub activity_id: String,
    pub activity_name: String,
    pub activity_token: String,
    pub person: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionError(String);

impl DefinitionError {
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DefinitionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DefinitionError {}

fn default_schema_version() -> u32 {
    1
}

impl ProcessDefinition {
    pub fn parse(value: serde_json::Value) -> Result<Self, DefinitionError> {
        let definition: Self = serde_json::from_value(value)
            .map_err(|error| DefinitionError(format!("invalid process definition: {error}")))?;
        definition.validate()?;
        Ok(definition)
    }

    pub fn validate(&self) -> Result<(), DefinitionError> {
        if self.schema_version != 1 {
            return Err(DefinitionError(format!(
                "unsupported process definition schemaVersion: {}",
                self.schema_version
            )));
        }
        if self.edition.code.trim().is_empty()
            || self.edition.name.trim().is_empty()
            || !self.edition.number.is_finite()
            || self.edition.number <= 0.0
        {
            return Err(DefinitionError(
                "edition code, name and positive number are required".to_string(),
            ));
        }

        let mut activities = HashMap::new();
        for activity in &self.activities {
            if activity.id.trim().is_empty() {
                return Err(DefinitionError("activity id is required".to_string()));
            }
            if activities.insert(activity.id.as_str(), activity).is_some() {
                return Err(DefinitionError(format!(
                    "duplicate activity id: {}",
                    activity.id
                )));
            }
            if activity.edition.as_deref().is_some_and(|edition| edition != self.edition.code) {
                return Err(DefinitionError(format!(
                    "activity {} belongs to a different edition",
                    activity.id
                )));
            }
        }

        let begin_count = self
            .activities
            .iter()
            .filter(|activity| activity.activity_type == ActivityType::Begin)
            .count();
        if begin_count != 1 {
            return Err(DefinitionError(format!(
                "process definition requires exactly one begin activity, found {begin_count}"
            )));
        }

        let mut route_ids = HashSet::new();
        for route in &self.routes {
            if route.id.trim().is_empty() || !route_ids.insert(route.id.as_str()) {
                return Err(DefinitionError(format!(
                    "route id must be non-empty and unique: {}",
                    route.id
                )));
            }
            if !activities.contains_key(route.from.as_str()) {
                return Err(DefinitionError(format!(
                    "route {} references unknown source activity {}",
                    route.id, route.from
                )));
            }
            if !activities.contains_key(route.to.as_str()) {
                return Err(DefinitionError(format!(
                    "route {} references unknown target activity {}",
                    route.id, route.to
                )));
            }
            if route.edition.as_deref().is_some_and(|edition| edition != self.edition.code) {
                return Err(DefinitionError(format!(
                    "route {} belongs to a different edition",
                    route.id
                )));
            }
            if route
                .waypoints
                .iter()
                .any(|point| !point.x.is_finite() || !point.y.is_finite())
            {
                return Err(DefinitionError(format!(
                    "route {} has an invalid waypoint",
                    route.id
                )));
            }
        }

        for activity in &self.activities {
            for route_id in &activity.route_list {
                let route = self.routes.iter().find(|route| route.id == *route_id);
                match route {
                    Some(route) if route.from == activity.id => {}
                    Some(_) => {
                        return Err(DefinitionError(format!(
                            "activity {} references route {} owned by another activity",
                            activity.id, route_id
                        )))
                    }
                    None => {
                        return Err(DefinitionError(format!(
                            "activity {} references unknown route {}",
                            activity.id, route_id
                        )))
                    }
                }
            }
        }

        for permission in &self.field_permissions {
            if permission.field.trim().is_empty() || !activities.contains_key(permission.activity.as_str()) {
                return Err(DefinitionError(format!(
                    "field permission references invalid field or activity: {} / {}",
                    permission.field, permission.activity
                )));
            }
            if permission.hidden && (permission.readable || permission.writable || permission.required) {
                return Err(DefinitionError(format!(
                    "hidden field permission cannot be readable, writable or required: {}",
                    permission.field
                )));
            }
            if permission.required && !permission.writable {
                return Err(DefinitionError(format!(
                    "required field permission must be writable: {}",
                    permission.field
                )));
            }
        }

        self.first_manual_task()?;
        Ok(())
    }

    pub fn first_manual_task(&self) -> Result<FirstManualTask, DefinitionError> {
        let begin = self
            .activities
            .iter()
            .find(|activity| activity.activity_type == ActivityType::Begin)
            .ok_or_else(|| DefinitionError("process definition has no begin activity".to_string()))?;
        let activities: HashMap<&str, &Activity> = self
            .activities
            .iter()
            .map(|activity| (activity.id.as_str(), activity))
            .collect();
        let routes: HashMap<&str, &ProcessRoute> = self
            .routes
            .iter()
            .map(|route| (route.id.as_str(), route))
            .collect();
        let mut queue = VecDeque::from([begin.id.as_str()]);
        let mut visited = HashSet::new();

        while let Some(activity_id) = queue.pop_front() {
            if !visited.insert(activity_id) {
                continue;
            }
            let activity = activities.get(activity_id).ok_or_else(|| {
                DefinitionError(format!("unknown activity in execution graph: {activity_id}"))
            })?;
            if activity.activity_type == ActivityType::Manual {
                let person = activity
                    .task_identity_list
                    .first()
                    .cloned()
                    .unwrap_or_default();
                return Ok(FirstManualTask {
                    activity_id: activity.id.clone(),
                    activity_name: if activity.name.trim().is_empty() {
                        activity.id.clone()
                    } else {
                        activity.name.clone()
                    },
                    activity_token: format!("{}:{}", self.edition.code, activity.id),
                    person,
                });
            }
            for route_id in &activity.route_list {
                if let Some(route) = routes.get(route_id.as_str()) {
                    queue.push_back(route.to.as_str());
                }
            }
            for route in self.routes.iter().filter(|route| {
                route.from == activity.id && !activity.route_list.contains(&route.id)
            }) {
                queue.push_back(route.to.as_str());
            }
        }

        Err(DefinitionError(
            "process definition has no manual activity reachable from begin".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_o2oa_graph_and_finds_first_manual() {
        let definition = ProcessDefinition::parse(json!({
            "schemaVersion": 1,
            "edition": {"code":"leave-v1","name":"Leave V1","number":1.0,"enabled":true},
            "activities": [
                {"id":"begin","activityType":"begin","routeList":["r1"]},
                {"id":"choice","activityType":"choice","routeList":["r2"]},
                {"id":"approve","activityType":"manual","name":"Manager approval","taskIdentityList":["manager@I"]}
            ],
            "routes": [
                {"id":"r1","from":"begin","to":"choice","waypoints":[{"x":1.0,"y":2.0}]},
                {"id":"r2","from":"choice","to":"approve"}
            ],
            "fieldPermissions": [
                {"field":"reason","activity":"approve","readable":true,"writable":true,"required":true}
            ]
        })).unwrap();

        let task = definition.first_manual_task().unwrap();
        assert_eq!(task.activity_id, "approve");
        assert_eq!(task.activity_token, "leave-v1:approve");
        assert_eq!(task.person, "manager@I");
    }

    // __PROCESS_DEFINITION_TEST_CONTINUATION__

