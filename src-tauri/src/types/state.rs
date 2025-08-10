use serde::{ de::DeserializeOwned, Deserialize, Serialize };
use serde_json::{ from_value, to_value, Value };
use specta::Type;
use strum::EnumIter;

#[derive(Serialize, Deserialize, Clone, Debug, Type, Default)]
pub struct State {
    pub current_project: Option<String>,
    pub color_scheme: ColorScheme,
    pub sidebar_width: u64,
    pub resource_manager_sidebar_width: u64
}

impl FromIterator<StateValue> for State {
    fn from_iter<T: IntoIterator<Item = StateValue>>(iter: T) -> Self {
        let mut state = Self::default();
        for i in iter {
            match i {
                StateValue::CurrentProject(value) => {
                    state.current_project = value;
                }
                StateValue::ColorScheme(value) => {
                    state.color_scheme = value;
                }
                StateValue::SidebarWidth(value) => {
                    state.sidebar_width = value;
                }
                StateValue::ResourceManagerSidebarWidth(value) => {
                    state.resource_manager_sidebar_width = value;
                }
            }
        }

        state
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum StateKey {
    CurrentProject,
    ColorScheme,
    SidebarWidth,
    ResourceManagerSidebarWidth
}

impl StateKey {
    pub fn key_name(&self) -> String {
        serde_json::to_string(self).unwrap().trim_matches('"').to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, Default)]
#[serde(rename_all = "snake_case")]
pub enum ColorScheme {
    Light,

    #[default]
    Dark,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case", tag = "key", content = "value")]
pub enum StateValue {
    CurrentProject(Option<String>),
    ColorScheme(ColorScheme),
    SidebarWidth(u64),
    ResourceManagerSidebarWidth(u64)
}

impl StateValue {
    pub fn key(&self) -> StateKey {
        match self {
            StateValue::CurrentProject(_) => StateKey::CurrentProject,
            StateValue::ColorScheme(_) => StateKey::ColorScheme,
            StateValue::SidebarWidth(_) => StateKey::SidebarWidth,
            StateValue::ResourceManagerSidebarWidth(_) => StateKey::ResourceManagerSidebarWidth
        }
    }

    pub fn key_name(&self) -> String {
        self.key().key_name()
    }

    pub fn value(&self) -> crate::Result<Value> {
        Ok(
            (match self {
                StateValue::CurrentProject(val) => to_value(val.clone()),
                StateValue::ColorScheme(color_scheme) => to_value(color_scheme.clone()),
                StateValue::SidebarWidth(width) => to_value(width.clone()),
                StateValue::ResourceManagerSidebarWidth(width) => to_value(width.clone())
            })?
        )
    }

    pub fn resolve<T: DeserializeOwned>(&self) -> crate::Result<T> {
        Ok(from_value::<T>(self.value()?)?)
    }

    pub fn wrap(key: StateKey, value: Value) -> crate::Result<Self> {
        Ok(match key {
            StateKey::CurrentProject => Self::CurrentProject(from_value(value)?),
            StateKey::ColorScheme => Self::ColorScheme(from_value(value)?),
            StateKey::SidebarWidth => Self::SidebarWidth(from_value(value)?),
            StateKey::ResourceManagerSidebarWidth => Self::ResourceManagerSidebarWidth(from_value(value)?)
        })
    }
}
