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
    pub fn get_double(&self) -> Option<f64> {
        unsafe {
            if self.raw.type_ == NT_Type_NT_DOUBLE {
                Some(self.raw.data.v_double)
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
