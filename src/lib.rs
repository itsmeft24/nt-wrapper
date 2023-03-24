pub mod bindings;
use bindings::*;
use std::ffi::CString;
pub struct NetworkTableInstance {
    handle: NT_Inst,
}

pub struct NetworkTableEntry {
    handle: NT_Entry,
}

pub struct Value {
    raw: NT_Value,
}
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self {
            raw: NT_Value {
                type_: NT_Type_NT_DOUBLE,
                last_change: 0,
                server_time: 0,
                data: NT_Value__bindgen_ty_1 { v_double: value },
            },
        }
    }
}
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            raw: NT_Value {
                type_: NT_Type_NT_INTEGER,
                last_change: 0,
                server_time: 0,
                data: NT_Value__bindgen_ty_1 { v_int: value },
            },
        }
    }
}
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self {
            raw: NT_Value {
                type_: NT_Type_NT_BOOLEAN,
                last_change: 0,
                server_time: 0,
                data: NT_Value__bindgen_ty_1 {
                    v_boolean: value as i32,
                },
            },
        }
    }
}
impl Value {
    pub fn default() -> Self {
        Self {
            raw: NT_Value {
                type_: NT_Type_NT_UNASSIGNED,
                last_change: 0,
                server_time: 0,
                data: NT_Value__bindgen_ty_1 { v_int: 0 },
            },
        }
    }
    pub fn get_int(&self) -> Option<i64> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_INTEGER {
                Some(self.raw.data.v_int)
            } else {
                None
            }
        }
    }
    pub fn get_double(&self) -> Option<f64> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_DOUBLE {
                Some(self.raw.data.v_double)
            } else {
                None
            }
        }
    }
    pub fn get_float(&self) -> Option<f32> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_FLOAT {
                Some(self.raw.data.v_float)
            } else {
                None
            }
        }
    }
    pub fn get_boolean(&self) -> Option<bool> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_BOOLEAN {
                Some(self.raw.data.v_boolean > 0)
            } else {
                None
            }
        }
    }
    pub fn get_string(&self) -> Option<String> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_STRING {
                let data = std::slice::from_raw_parts(
                    self.raw.data.v_string.str_ as *const u8,
                    self.raw.data.v_string.len as usize,
                );
                match std::string::String::from_utf8(data.to_vec()) {
                    Ok(str) => Some(str),
                    Err(_) => None,
                }
            } else {
                None
            }
        }
    }
    pub fn get_raw(&self) -> Option<Vec<u8>> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_RAW {
                let data =
                    std::slice::from_raw_parts(self.raw.data.v_raw.data, self.raw.data.v_raw.size);
                Some(data.to_vec())
            } else {
                None
            }
        }
    }
}

impl NetworkTableEntry {
    pub fn get_value(&self) -> Option<Value> {
        let mut value = Value::default();
        unsafe { NT_GetEntryValue(self.handle, &mut value.raw as *mut NT_Value) }
        if value.raw.type_ != NT_Type_NT_UNASSIGNED {
            Some(value)
        } else {
            None
        }
    }
    pub fn set_value(&mut self, value: Value) {
        unsafe {
            NT_SetEntryValue(self.handle, &value.raw as *const NT_Value);
        }
    }
}

impl Drop for NetworkTableInstance {
    fn drop(&mut self) {
        unsafe { NT_DestroyInstance(self.handle) }
    }
}

impl NetworkTableInstance {
    pub fn get_default() -> NetworkTableInstance {
        NetworkTableInstance {
            handle: unsafe { NT_GetDefaultInstance() },
        }
    }
    pub unsafe fn get_handle(&self) -> NT_Inst {
        self.handle
    }
    pub fn start_client_3(&mut self, client: &str) {
        let client = CString::new(client).unwrap();
        unsafe { NT_StartClient3(self.handle, client.as_ptr()) };
    }
    pub fn start_client_4(&mut self, client: &str) {
        let client = CString::new(client).unwrap();
        unsafe { NT_StartClient4(self.handle, client.as_ptr()) };
    }
    pub fn set_server(&mut self, host_name: &str, port: u32) {
        let host_name = CString::new(host_name).unwrap();
        unsafe { NT_SetServer(self.handle, host_name.as_ptr(), port) };
    }
    pub fn set_server_team(&mut self, team_number: u32, port: u32) {
        unsafe { NT_SetServerTeam(self.handle, team_number, port) };
    }
    pub fn start_driver_station_client(&mut self, port: u32) {
        unsafe { NT_StartDSClient(self.handle, port) };
    }
    pub fn is_connected(&self) -> bool {
        unsafe { NT_IsConnected(self.handle) > 0 }
    }
    pub fn get_entry(&self, table: &str) -> NetworkTableEntry {
        let table_cstr = CString::new(table).unwrap();
        unsafe {
            NetworkTableEntry {
                handle: NT_GetEntry(self.handle, table_cstr.as_ptr(), table.len()),
            }
        }
    }
}
