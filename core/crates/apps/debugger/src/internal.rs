use crate::*;

pub struct DebuggerInternal {
    pub(crate) debug_time: HuskyDebugTime,
    pub(crate) config: DebuggerConfig,
}

// impl DebuggerInternal {
//     pub fn subtraces(
//         &mut self,
//         trace_id: TraceId,
//         effective_opt_input_id: Option<usize>,
//     ) -> Avec<Trace> {
//         let trace = self.debug_time.trace(trace_id);
//         let subtraces = self.debug_time.subtraces(trace_id, effective_opt_input_id);
//         self.debug_time
//             .set_subtraces(&trace, effective_opt_input_id, &subtraces);
//         subtraces
//     }

//     pub fn expansions(&self) -> &HashMap<TraceId, bool> {
//         self.runtime.expansions()
//     }

//     pub fn showns(&self) -> &HashMap<TraceId, bool> {
//         self.runtime.showns()
//     }

//     pub fn activate(&mut self, id: TraceId) {
//         self.debug_time.active_trace_id = Some(id);
//     }

//     pub fn toggle_expansion(&mut self, id: TraceId) {
//         self.runtime.toggle_expansion(id)
//     }

//     pub fn toggle_show(&mut self, id: TraceId) {
//         self.runtime.toggle_show(id)
//     }

//     pub fn trace(&self, id: TraceId) -> Arc<Trace> {
//         self.runtime.trace(id)
//     }

//     pub fn decode_focus(&self, command: &str) -> JsonResult<Focus> {
//         self.runtime.decode_focus(command)
//     }

//     pub fn lock_input(&mut self, input_str: &str) -> (Option<Option<usize>>, Option<String>) {
//         self.runtime.lock_input(input_str)
//     }

//     pub fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk> {
//         self.runtime.trace_stalk(trace_id, input_id)
//     }
// }
